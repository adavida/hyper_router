use quote::{__private::TokenStream, format_ident};

use crate::config::RouteConfig;
use crate::paths::Paths;

pub(crate) fn make_route(config: &RouteConfig) -> TokenStream {
    let method = format_ident!("{}", &config.method.to_string());

    let paths_extractor = Paths::new(&config.path);

    let (vars, paths) = paths_extractor.get_data();

    let path = quote::quote! { [ #(#paths), *]};
    let vars_controller = quote::quote! {#(#vars),*};
    let controller_name = format_ident!("{}", &config.controller_name);

    quote::quote! {
        (&hyper::Method::#method, #path ) => {
            let text: String = controller::#controller_name(#vars_controller);
            Ok(hyper::Response::new(text.into()))
        }
    }
}

pub fn make_route_404() -> TokenStream {
    quote::quote! {
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_simple_404() {
        let result = quote::quote! {
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        };

        assert_eq!(make_route_404().to_string(), result.to_string());
    }

    #[test]
    fn test_simple_get_route_return_text() {
        let result = quote::quote! {
            (&hyper::Method::GET, ["path"]) => {
                let text: String = controller::path_controller();
                Ok(hyper::Response::new(text.into()))
            }
        };
        let config = RouteConfig {
            method: hyper::Method::from_str("GET").unwrap(),
            path: "/path".to_string(),
            controller_name: "path_controller".to_string(),
        };
        assert_eq!(make_route(&config).to_string(), result.to_string());
    }

    #[test]
    fn test_simple_get_route_return_text_with_var_in_path() {
        let result = quote::quote! {
            (&hyper::Method::GET, ["path", var1]) => {
                let text: String = controller::path_controller(var1);
                Ok(hyper::Response::new(text.into()))
            }
        };
        let config = RouteConfig {
            method: hyper::Method::from_str("GET").unwrap(),
            path: "/path/{var1}".to_string(),
            controller_name: "path_controller".to_string(),
        };
        assert_eq!(make_route(&config).to_string(), result.to_string());
    }
}
