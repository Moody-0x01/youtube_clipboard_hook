use crate::options as opt;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[allow(dead_code)]
pub fn load_config(path: &str) -> Result<opt::Options, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut config: opt::Options = serde_json::from_reader(reader)?;
    config.setupdefault_formats();
    Ok(config)
}
