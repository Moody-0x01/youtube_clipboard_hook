use std::os::unix::net::UnixStream;
use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};

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

    pub fn init_daemon(socket_path: &str) -> io::Result<()> {
        let stream = UnixStream::connect(socket_path)?;
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
