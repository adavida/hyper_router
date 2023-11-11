mod config;
mod entry_config;
mod paths;
mod route;

use crate::config::{Config, RouteConfig};
use crate::route::make_route;
use proc_macro::TokenStream;
use std::fs;

#[proc_macro]
pub fn generate_router(input: TokenStream) -> TokenStream {
    let input_config = entry_config::EntryConfig::extract(proc_macro2::TokenStream::from(input));

    let yaml = fs::read_to_string(input_config.filename).unwrap();
    let config = Config::from_yaml(yaml.as_str());

    TokenStream::from(generate_router_fn(config.paths))
}

fn generate_router_fn(paths_config: Vec<RouteConfig>) -> proc_macro2::TokenStream {
    let route_404 = route::make_route_404();
    let elements = paths_config.into_iter().map(|x| make_route(&x));
    quote::quote! {
        async fn route(req: hyper::Request<hyper::Body>)-> Result<hyper::Response<hyper::Body>, hyper::http::Error>{
            let path = req.uri().path().split('/').collect::<Vec<&str>>();
            match (req.method(), path.as_slice()) {
                #(#elements)*
                #route_404
            }
        }
    }
}

