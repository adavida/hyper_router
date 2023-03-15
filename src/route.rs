use quote::{__private::TokenStream, format_ident};

pub struct RouteConfig {
    method: String,
    path: String,
    controller_name: String,
}

pub fn make_route(config: &RouteConfig) -> TokenStream {
    let method = format_ident!("{}", &config.method);
    let path = &config.path;
    let controller_name = format_ident!("{}", &config.controller_name);

    quote::quote! {
        (&hyper::Method::#method, #path ) => {
            let text: String = controller::#controller_name();
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
    use crate::route::*;

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
            (&hyper::Method::GET, "/path") => {
                let text: String = controller::path_controller();
                Ok(hyper::Response::new(text.into()))
            }
        };
        let config = RouteConfig {
            method: "GET".to_string(),
            path: "/path".to_string(),
            controller_name: "path_controller".to_string(),
        };
        assert_eq!(make_route(&config).to_string(), result.to_string());
    }
}
