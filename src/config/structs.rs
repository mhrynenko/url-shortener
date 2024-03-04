use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DataBase {
    pub url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Listener {
    pub host: String,
    pub port: usize,
}