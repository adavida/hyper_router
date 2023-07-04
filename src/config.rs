use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Config {
    #[serde(default = "Vec::new")]
    pub(crate) paths: Vec<RouteConfig>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct RouteConfig {
    #[serde(deserialize_with = "hyper_serde::deserialize")]
    pub(crate) method: hyper::Method,
    pub(crate) path: String,
    pub(crate) controller_name: String,
}

impl Config {
    pub(crate) fn from_yaml(entry: &str) -> Self {
        serde_yaml::from_str(entry).unwrap()
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
        path: /path1
        controller_name: controller1
    -
        method: POST
        path: /path2
        controller_name: controller2
        ";
        let result = Config::from_yaml(entry);
        let expected = Config {
            paths: vec![
                RouteConfig {
                    method: hyper::Method::GET,
                    path: "/path1".to_string(),
                    controller_name: "controller1".to_string(),
                },
                RouteConfig {
                    method: hyper::Method::POST,
                    path: "/path2".to_string(),
                    controller_name: "controller2".to_string(),
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