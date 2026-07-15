import asyncio
import json
from fastapi import FastAPI
from fastapi.responses import HTMLResponse, StreamingResponse
from pydantic import BaseModel
from os import environ, makedirs, path, remove

app = FastAPI()
CONFIG_FILE="/home/moody/.config/clippy_hook/config.json"
DEFAULT_PATH = environ["HOME"] + "/" + "clippy_hook"
SOCKET_PATH = "/tmp/clippy_hook.sock"
daemon_queue = asyncio.Queue()

if not path.exists(DEFAULT_PATH): makedirs(DEFAULT_PATH, exist_ok=True)

class ConfigSchema(BaseModel):
    use_youtube: bool
    active: bool
    use_wget: bool
    use_soundcloud: bool
    use_transmission: bool
    quiet: bool
    download_path: str
    formats: list[str]
    download_path_set: bool

@app.get("/", response_class=HTMLResponse)
def get_ui():
    with open("./views/index.html", "r") as f:
        return HTMLResponse(content=f.read())

@app.get("/api/config", response_model=ConfigSchema)
def get_config():
    with open(CONFIG_FILE, "r") as f:
        return json.load(f)

@app.post("/api/config")
def update_config(updated_data: ConfigSchema):
    with open(CONFIG_FILE, "w") as f:
        new_config = updated_data.model_dump()
        if new_config["download_path"] == "":
            new_config["download_path"] = DEFAULT_PATH;
        json.dump(new_config, f, indent=4)
        print("Config was dumped to: ", CONFIG_FILE)
    return {"status": "success"}


async def handle_daemon_client(reader, writer):
    print("Rust daemon connected via Unix Socket!")
    try:
        while True:
            data = await reader.readline()
            if not data:
                break
            message = data.decode()
            await daemon_queue.put(message)
    except Exception as e:
        print(f"Socket error: {e}")
    finally:
        writer.close()

@app.on_event("startup")
async def startup_event():
    if path.exists(SOCKET_PATH):
        remove(SOCKET_PATH)
    server = await asyncio.start_unix_server(handle_daemon_client, path=SOCKET_PATH)
    asyncio.create_task(server.serve_forever())

async def event_generator():
    try:
        while True:
            message = await daemon_queue.get()
            yield f"data: {message}\n\n"
            daemon_queue.task_done()
    except asyncio.CancelledError:
        print("Client disconnected.")

@app.get("/stream")
async def stream_daemon_data():
    return StreamingResponse(event_generator(), media_type="text/event-stream")
