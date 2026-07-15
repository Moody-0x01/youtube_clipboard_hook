use std::os::unix::net::UnixStream;
use std::time::Duration;
use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};
use std::thread;

enum LogTarget {
    Terminal(io::Stdout),
    Socket(UnixStream),
}

pub struct GlobalLogger {
    target: Mutex<LogTarget>,
}

static LOGGER: OnceLock<GlobalLogger> = OnceLock::new();
impl GlobalLogger {
    pub fn init_cli() {
        let logger = GlobalLogger {
            target: Mutex::new(LogTarget::Terminal(io::stdout())),
        };
        let _ = LOGGER.set(logger);
    }

    fn connect_to_backend(socket_path: &str) -> UnixStream {
        loop {
            match UnixStream::connect(socket_path) {
                Ok(stream) => {
                    println!("[clippy_hook] Connected to FastAPI socket successfully!");
                    return stream;
                }
                Err(e) => {
                    eprintln!("[clippy_hook] Socket server not ready ({:?}). Retrying in 1s...", e.kind());
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
    pub fn init_daemon(socket_path: &str) -> io::Result<()> {
        let stream = GlobalLogger::connect_to_backend(socket_path);
        let logger = GlobalLogger {
            target: Mutex::new(LogTarget::Socket(stream)),
        };
        if LOGGER.set(logger).is_err() {
            println!("Logger was already initialized!");
        }
        Ok(())
    }

    pub fn log(message: &str) {
        if let Some(logger) = LOGGER.get() {
            if let Ok(mut guard) = logger.target.lock() {
                let formatted = format!("{}\n", message);
                
                match &mut *guard {
                    LogTarget::Terminal(stdout) => {
                        let mut handle = stdout.lock();
                        let _ = handle.write_all(formatted.as_bytes());
                        let _ = handle.flush();
                    }
                    LogTarget::Socket(stream) => {
                        let _ = stream.write_all(formatted.as_bytes());
                        println!("[GlobalLogger::log] Sent log to socket: {}", message);
                        let _ = stream.flush();
                    }
                }
            }
        } else {
            // Fallback if someone logs before initialization
            eprintln!("[UNINITIALIZED LOGGER]: {}", message);
        }
    }
}
