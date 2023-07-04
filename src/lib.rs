mod config;
mod entry_config;
mod paths;
mod route;

use crate::config::{Config, RouteConfig};
use crate::route::make_route;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_router(input: TokenStream) -> TokenStream {
    let input_config = entry_config::EntryConfig::extract(proc_macro2::TokenStream::from(input));

    let config = Config::from_yaml(&input_config.filename);

    TokenStream::from(generate_router_fn(config.paths))
}

fn generate_router_fn(paths_config: Vec<RouteConfig>) -> proc_macro2::TokenStream {
    let route_404 = route::make_route_404();
    let elements = paths_config.into_iter().map(|x| make_route(&x));
    quote::quote! {
        fn route(req: hyper::Request<hyper::Body>)-> Result<hyper::Response<hyper::Body>, hyper::Error>{
            let path = req.uri().path().split('/').collect::<Vec<&str>>();
            match (req.method(), path.as_slice()) {
                #(#elements)*
                #route_404
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::{config::*, *};

    #[test]
    fn only_default_404() {
        let expeded = quote::quote! {
            fn route(req: hyper::Request<hyper::Body>)-> Result<hyper::Response<hyper::Body>, hyper::Error>{
                let path = req.uri().path().split('/').collect::<Vec<&str>>();
                match (req.method(), path.as_slice()) {
                    _ => {
                            let mut not_found = hyper::Response::default();
                            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
                            Ok(not_found)
                    }
                }
            }
        };
        let result = generate_router_fn(vec![]);
        assert_eq!(expeded.to_string(), result.to_string());
    }

    #[test]
    fn with_root_path() {
        let expeded = quote::quote! {
            fn route(req: hyper::Request<hyper::Body>)-> Result<hyper::Response<hyper::Body>, hyper::Error>{
                let path = req.uri().path().split('/').collect::<Vec<&str>>();
                match (req.method(), path.as_slice()) {
                    (&hyper::Method::GET, [] ) => {
                        let text: String = controller::route_controller();
                        Ok(hyper::Response::new(text.into()))
                    }
                    (&hyper::Method::POST, ["post", "url", var, var2] ) => {
                        let text: String = controller::route_controller2(var, var2);
                        Ok(hyper::Response::new(text.into()))
                    }
                    _ => {
                            let mut not_found = hyper::Response::default();
                            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
                            Ok(not_found)
                    }
                }
            }
        };
        let config = vec![
            RouteConfig {
                method: hyper::Method::GET,
                path: "/".to_string(),
                controller_name: "route_controller".to_string(),
            },
            RouteConfig {
                method: hyper::Method::POST,
                path: "/post/url/{var}/{var2}".to_string(),
                controller_name: "route_controller2".to_string(),
            },
        ];
        let result = generate_router_fn(config);
        assert_eq!(expeded.to_string(), result.to_string());
    }
}
