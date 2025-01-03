use serde::{de, Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Config {
    #[serde(default = "Vec::new")]
    pub(crate) paths: Vec<RouteConfig>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct RouteConfig {
    #[serde(deserialize_with = "deserialize_from_str")]
    pub(crate) method: hyper::Method,
    pub(crate) path: String,
    pub(crate) controller_name: String,
    #[serde(default)]
    pub(crate) get_query_params: bool,
    #[serde(default)]
    pub(crate) get_post_data: bool,
}
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

impl Default for RouteConfig {
    fn default() -> Self {
        Self {
            method: hyper::Method::GET,
            path: "/path/unused".to_string(),
            controller_name: "unused_controler".to_string(),
            get_query_params: false,
            get_post_data: false,
        }
    }
}

impl Config {
    pub(crate) fn from_yaml(entry: &str) -> Self {
        serde_yml::from_str(entry).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_yaml() {
        let entry = "\
paths:
    -
        method: GET
        path: /path/unused
        controller_name: unused_controler
    -
        method: GET
        path: /path1
        controller_name: controller1
        get_query_params: true
    -
        method: POST
        path: /path2
        controller_name: controller2
        get_post_data: true
        ";
        let result = Config::from_yaml(entry);
        let expected = Config {
            paths: vec![
                RouteConfig::default(),
                RouteConfig {
                    method: hyper::Method::GET,
                    path: "/path1".to_string(),
                    controller_name: "controller1".to_string(),
                    get_query_params: true,
                    ..RouteConfig::default()
                },
                RouteConfig {
                    method: hyper::Method::POST,
                    path: "/path2".to_string(),
                    controller_name: "controller2".to_string(),
                    get_post_data: true,
                    ..RouteConfig::default()
                },
            ],
        };
        assert_eq!(expected, result);
    }
    #[test]
    fn deserialize_void_yaml() {
        let entry = "";
        let result = Config::from_yaml(entry);
        let expected = Config { paths: vec![] };
        assert_eq!(expected, result);
    }
}
