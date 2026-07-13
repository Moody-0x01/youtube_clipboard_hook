import json
from fastapi import FastAPI
from fastapi.responses import HTMLResponse
from pydantic import BaseModel
from os import environ, makedirs, path

app = FastAPI()
CONFIG_FILE = "/home/moody/.config/cphook/config.json"
DEFAULT_PATH = environ["HOME"] + "/" + "cphook"

if not path.exists(DEFAULT_PATH): makedirs(DEFAULT_PATH, exist_ok=True)


class ConfigSchema(BaseModel):
    use_youtube: bool
    use_mpv: bool
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
        # print(type (updated_data.model_dump()))
        if new_config["download_path"] == "":
            new_config["download_path"] = DEFAULT_PATH;

        json.dump(new_config, f, indent=4)
    return {"status": "success"}
