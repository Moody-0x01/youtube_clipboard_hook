use arboard::Error;
use std::process::exit;

pub fn on_error(e: Error, func: &str)
{
    eprintln!("Source: {}", func);
    match e {
    Error::ContentNotAvailable => {},
    Error::ClipboardNotSupported => {
        eprintln!("ClipboardNotSupported");
        exit(1);
    },
    Error::ClipboardOccupied => {
        eprintln!("ClipboardOccupied");
        exit(1);
    },
    Error::ConversionFailure => {
        eprintln!("ConversionFailure");
        exit(1);
    },
    Error::Unknown { description } => {
        eprintln!("Unknown: {}", description);
        exit(1);
    },
    _ => {
        eprintln!("an unexpected error was returned");
        exit(1);
    }
    }
}
