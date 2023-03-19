use quote::{__private::TokenStream, format_ident};

pub struct RouteConfig {
    method: String,
    path: String,
    controller_name: String,
}

#[derive(Debug)]
enum PathElement<'a> {
    Name(&'a str),
    Var(&'a str),
}

impl PathElement<'_> {
    fn from_str(entry: &str) -> Option<PathElement> {
        match entry.trim() {
            "" => None,
            a if a.starts_with("{") && a.ends_with("}") => {
                let len = a.len() - 1;
                Some(PathElement::Var(&a[1..len]))
            }
            a => Some(PathElement::Name(a)),
        }
    }

    fn to_token_tree(&self) -> proc_macro2::TokenTree {
        match self {
            Self::Name(name) => proc_macro2::TokenTree::from(proc_macro2::Literal::string(name)),
            Self::Var(var) => proc_macro2::TokenTree::from(quote::format_ident!("{var}")),
        }
    }
}

pub fn make_route(config: &RouteConfig) -> TokenStream {
    let method = format_ident!("{}", &config.method);

    let paths = &config
        .path
        .split("/")
        .filter_map(PathElement::from_str)
        .collect::<Vec<PathElement>>();
    let paths = paths.iter().map(|v| v.to_token_tree());
    let vars = paths.clone().filter(|v| match v {
        proc_macro2::TokenTree::Ident(_) => true,
        _ => false,
    });

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
            (&hyper::Method::GET, ["path"]) => {
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

    #[test]
    fn test_simple_get_route_return_text_with_var_in_path() {
        let result = quote::quote! {
            (&hyper::Method::GET, ["path", var1]) => {
                let text: String = controller::path_controller(var1);
                Ok(hyper::Response::new(text.into()))
            }
        };
        let config = RouteConfig {
            method: "GET".to_string(),
            path: "/path/{var1}".to_string(),
            controller_name: "path_controller".to_string(),
        };
        assert_eq!(make_route(&config).to_string(), result.to_string());
    }
}
