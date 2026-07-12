use crate::options as opt;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[allow(dead_code)]
pub fn load_config(path: &str) -> Result<opt::Options, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    
    Ok(config)
}
