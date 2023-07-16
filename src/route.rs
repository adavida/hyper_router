use quote::{__private::TokenStream, format_ident};

use crate::config::RouteConfig;
use crate::paths::Paths;

pub(crate) fn make_route(config: &RouteConfig) -> TokenStream {
    let method = format_ident!("{}", &config.method.to_string());

    let paths_extractor = Paths::new(&config.path);

    let (vars, paths) = paths_extractor.get_data();

    let binding_get_params = &config.get_params.clone().map(|str | {
        let name_var = to_token_tree("get_vars");
        let type_var = to_token_tree(&str.as_ref());
        let quote = quote::quote! {
            let get_vars: Result<controller::#type_var, String> =  serde_qs::from_str(req.uri().query().unwrap_or(""))
                .map_err(|x| x.to_string());
        };
        (name_var, quote)
    });

    let (name_vars, create_vars): (Vec<_>, Vec<_>) = vec![binding_get_params]
        .into_iter()
        .flatten()
        .map(|x| x.clone())
        .unzip();

    let vars = vars.chain(name_vars.clone().into_iter());

    let controller_name = format_ident!("{}", &config.controller_name);

    let ok_quote = quote::quote! {
        let text: String = controller::#controller_name(#(#vars),*);
        Ok(hyper::Response::new(text.into()))
    };
    let inner_match = if name_vars.is_empty() {
        quote::quote! {
            #ok_quote
        }
    } else {
        quote::quote! {
            #(#create_vars); *
            match (#(#name_vars), *) {
                (#(Ok(#name_vars)), *) =>  {
                    #ok_quote
                }
                _ => {
                    let errors = vec![ #(#name_vars), *];
                    let errors = errors.into_iter().filter_map(|x| x.err());

                    hyper::Response::builder()
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .body(errors.collect::<Vec<_>>().join(",").into())
                }
            }
        }
    };

    quote::quote! {
        (&hyper::Method::#method, [#(#paths), *]) => {
            #inner_match
        }
    }
}

fn to_token_tree(entry: &str) -> proc_macro2::TokenTree {
    proc_macro2::TokenTree::from(quote::format_ident!("{}", &entry))
}

pub fn make_route_404() -> TokenStream {
    quote::quote! {
        _ => {
            let mut not_found = hyper::Response::default();
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
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
                let mut not_found = hyper::Response::default();
                *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        };

        assert_eq!(make_route_404().to_string(), result.to_string());
    }

    #[test]
    fn test_simple_get_route_return_text() {
        let result = quote::quote! {
            (&hyper::Method::GET, ["", "path"]) => {
                let text: String = controller::path_controller();
                Ok(hyper::Response::new(text.into()))
            }
        };
        let config = RouteConfig {
            method: hyper::Method::from_str("GET").unwrap(),
            path: "/path".to_string(),
            controller_name: "path_controller".to_string(),
            ..Default::default()
        };
        assert_eq!(make_route(&config).to_string(), result.to_string());
    }

    #[test]
    fn test_simple_get_route_return_text_with_var_in_path() {
        let result = quote::quote! {
            (&hyper::Method::GET, ["", "path", var1]) => {
                let text: String = controller::path_controller(var1);
                Ok(hyper::Response::new(text.into()))
            }
        };
        let config = RouteConfig {
            method: hyper::Method::from_str("GET").unwrap(),
            path: "/path/{var1}".to_string(),
            controller_name: "path_controller".to_string(),
            ..Default::default()
        };
        assert_eq!(make_route(&config).to_string(), result.to_string());
    }
}
