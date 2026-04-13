use arboard::Error;
use std::process::exit;

pub fn on_error(e: Error, func: &str)
{
    match e {
    Error::ContentNotAvailable => {},
    Error::ClipboardNotSupported => {
        eprintln!("Source: {}", func);
        eprintln!("ClipboardNotSupported");
        exit(1);
    },
    Error::ClipboardOccupied => {
        eprintln!("Source: {}", func);
        eprintln!("ClipboardOccupied");
        exit(1);
    },
    Error::ConversionFailure => {
        eprintln!("Source: {}", func);
        eprintln!("ConversionFailure");
        exit(1);
    },
    Error::Unknown { description } => {
        eprintln!("Source: {}", func);
        eprintln!("Unknown: {}", description);
        exit(1);
    },
    _ => {
        eprintln!("Source: {}", func);
        eprintln!("an unexpected error was returned");
        exit(1);
    }
    }
}
