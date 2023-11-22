use http_body_util::BodyExt;
use hyper::{Request, StatusCode};

hyper_router::generate_router!(filename: "./tests/simple_routing.yml");

mod controller {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct EchoParm {
        param1: Option<String>,
        #[serde(default = "default_param2")]
        param2: String,
    }

    pub fn get_root() -> String {
        "root get page".to_string()
    }

    pub fn post_root(entry: EchoParm) -> String {
        let param1 = entry.param1.unwrap_or("default_param1".to_string());
        format!("root post page {} {}", param1, entry.param2)
    }

    pub fn get_root_var(entry: &str) -> String {
        format!("var : {entry}")
    }

    pub fn get_root_two_var(var1: &str, var2: &str) -> String {
        format!("var1 : {var1} - var2 : {var2}")
    }

    fn default_param2() -> String {
        "default_param2".to_string()
    }
}

#[tokio::test]
async fn on_am_existing_get_route() {
    let request = Request::builder()
        .uri("http://localhost/")
        .method("GET")
        .body(String::new())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "root get page");
    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn on_am_existing_get_route_with_var() {
    let request = Request::builder()
        .uri("http://localhost/message")
        .method("GET")
        .body(String::new())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "var : message");
    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn on_am_existing_get_route_with_var_whit_another_value() {
    let request = Request::builder()
        .uri("http://localhost/another_message")
        .method("GET")
        .body(String::new())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "var : another_message");
    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn on_am_existing_get_route_with_two_var() {
    let request = Request::builder()
        .uri("http://localhost/var/message/123")
        .method("GET")
        .body(String::new())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "var1 : message - var2 : 123");
    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn on_a_non_existing_route() {
    let request = Request::builder()
        .uri("http://localhost/no/existe")
        .method("GET")
        .body(String::new())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "");
    assert_eq!(parts.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn on_am_existing_post_route() {
    let request = Request::builder()
        .uri("http://localhost/")
        .method("POST")
        .body(String::new())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "root post page default_param1 default_param2");
    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn on_am_existing_post_route_with_params() {
    let request = Request::builder()
        .uri("http://localhost/")
        .method("POST")
        .body("param1=123&param2=456".to_string())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();
    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "root post page 123 456");
    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn on_am_existing_post_route_with_another_params() {
    let request = Request::builder()
        .uri("http://localhost/")
        .method("POST")
        .body("param1=p1&param2=p2".to_string())
        .unwrap();

    let response = route(request).await.unwrap();

    let (parts, body) = response.into_parts();

    let body_string = body.collect().await.unwrap().to_bytes();

    assert_eq!(body_string, "root post page p1 p2");
    assert_eq!(parts.status, StatusCode::OK);
}
