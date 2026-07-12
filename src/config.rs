// use std::fs::File;
// use crate::options::{Options};
// use std::io::BufReader;
//
// fn load_config(path: &str) -> Result<Options, Box<dyn std::error::Error>>
// {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let config = serde_json::from_reader(reader)?;
//     Ok(config)
// }
