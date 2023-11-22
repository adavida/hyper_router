use quote::{__private::TokenStream, format_ident};

use crate::config::RouteConfig;
use crate::paths::Paths;

pub(crate) fn make_route(config: &RouteConfig) -> TokenStream {
    let method = format_ident!("{}", &config.method.to_string());

    let paths_extractor = Paths::new(&config.path);

    let (vars, paths) = paths_extractor.get_data();

    let binding_query_params = if config.get_query_params {
        let name_var = to_token_tree("get_vars");
        let quote = quote::quote! {
            let get_vars: Result<_, String> =  serde_qs::from_str(req.uri().query().unwrap_or(""))
                .map_err(|x| x.to_string());
        };
        Some((name_var, quote))
    } else {
        None
    };

    let binding_post_data = if config.get_post_data {
        let name_var = to_token_tree("get_body");
        let quote = quote::quote! {
            let body = req.collect().await.unwrap().to_bytes();
            let get_body: Result<_, String> = serde_qs::from_bytes(body.as_ref()).map_err(|_| "".to_string());
        };
        Some((name_var, quote))
    } else {
        None
    };

    let (name_vars, create_vars): (Vec<_>, Vec<_>) = vec![binding_query_params, binding_post_data]
        .into_iter()
        .flatten()
        .unzip();

    let vars = vars.chain(name_vars.clone().into_iter());

    let controller_name = format_ident!("{}", &config.controller_name);

    let ok_quote = quote::quote! {
        let text: String = controller::#controller_name(#(#vars),*);
        Ok(hyper::Response::new(http_body_util::Full::new(text.into())))
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
