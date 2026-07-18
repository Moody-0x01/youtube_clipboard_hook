use arboard::Error;
use std::process::exit;

pub fn on_error(e: Error, func: &str)
{
    match e {
    Error::ContentNotAvailable => { return ; },
    Error::ClipboardNotSupported => {
        eprintln!("Source: {}", func);
        eprintln!("ClipboardNotSupported");
    },
    Error::ClipboardOccupied => {
        eprintln!("Source: {}", func);
        eprintln!("ClipboardOccupied");
        return ;
    },
    Error::ConversionFailure => {
        eprintln!("Source: {}", func);
        eprintln!("ConversionFailure");
        return ;
    },
    Error::Unknown { description } => {
        eprintln!("Source: {}", func);
        eprintln!("Unknown: {}", description);
        return ;
    },
    _ => {
        eprintln!("Source: {}", func);
        eprintln!("an unexpected error was returned");
    }
    }
    exit(1);
}