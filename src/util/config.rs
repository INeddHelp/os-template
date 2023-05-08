use std::collections::HashMap;

pub struct Config {
    config_map: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            config_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.config_map.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.config_map.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.config_map.remove(key)
    }
}
