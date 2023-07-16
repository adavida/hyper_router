use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EchoParm {
    param1: String,
    param2: String,
}
pub fn hello_world() -> String {
    "Hello, World".to_string()
}

pub fn echo(entry: &str, pv: EchoParm) -> String {
    format!("{} ,  {:?}", entry.to_string(), pv)
}
