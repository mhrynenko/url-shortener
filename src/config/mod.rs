mod structs;

use std::io::{Read};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;

#[derive(Default)]
pub struct Config {
    pub db: structs::DataBase,
    pub listener: structs::Listener,

    yaml: Value,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let mut cfg = Self::read().unwrap();
        cfg.db = cfg.parse("db").unwrap();
        cfg.listener = cfg.parse("listener").unwrap();

        Ok(cfg)
    }

    fn read() -> Result<Config, Box<dyn Error>>{
        let path = std::path::Path::new("config.yaml");
        let display = path.display();

        let mut file = match std::fs::File::open(&path) {
            Err(why) => return Err(Box::from(
                format!("couldn't open {}: {:?}", display, why)),
            ),
            Ok(file) => file,
        };

        let mut config_str = String::new();
        match file.read_to_string(&mut config_str) {
            Err(why) => return Err(Box::from(
                format!("couldn't read {}: {:?}", display, why)),
            ),
            Ok(..) => ()
        }

        let val = match serde_yaml::from_str::<Value>(&config_str) {
            Ok(val) => val,
            Err(e) => return Err(Box::from(
                format!("failed to get value from config string {:?}", e),
            )),
        };

        let mut cfg: Config = Default::default();
        cfg.yaml = val;

        Ok(cfg)
    }



    fn parse<T>(&self, yaml_key: &str) -> Result<T, Box<dyn Error>>
        where T: DeserializeOwned
    {
        let element = match self.yaml.get(yaml_key) {
            None => return Err(Box::from(format!("failed to find '{yaml_key}' in config"))),
            Some(value) => value,
        };

        let parsed: T = match serde_json::from_str(Value::to_string(element).as_str()) {
            Ok(v) => v,
            Err(e) => return Err(Box::from(
                format!("failed to get structure from '{yaml_key}' {:?}", e),
            )),
        };

        Ok(parsed)
    }
}
