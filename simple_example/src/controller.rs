use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EchoParm {
    param1: String,
    param2: Option<String>,
    #[serde(default = "default_param3")]
    param3: String,
}
fn default_param3() -> String {
    "param3".to_string()
}

pub fn hello_world() -> String {
    "Hello, World".to_string()
}

pub fn echo(entry: &str, pv: EchoParm) -> String {
    format!("{} ,  {:?}", entry, pv)
}
