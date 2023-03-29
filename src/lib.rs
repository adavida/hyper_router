use proc_macro::TokenStream;

mod paths;
mod route;

#[proc_macro]
pub fn generate_router(_item: TokenStream) -> TokenStream {
    TokenStream::from(generate_router_fn())
}

fn generate_router_fn() -> proc_macro2::TokenStream {
    let route_404 = route::make_route_404();
    quote::quote! {
        fn route(req: hyper::Resquest<hyper::Body>)-> Result<hyper::Response<hyper::Body>, hyper::Error>{
            match(req.method(), req.uri().path()) {
                #route_404
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn only_default_404() {
        let expeded = quote::quote! {
            fn route(req: hyper::Resquest<hyper::Body>)-> Result<hyper::Response<hyper::Body>, hyper::Error>{
                match (req.method(), req.uri().path()) {
                    _ => {
                            let mut not_found = Response::default();
                            *not_found.status_mut() = StatusCode::NOT_FOUND;
                            Ok(not_found)
                    }
                }
            }
        };
        let result = generate_router_fn();
        assert_eq!(expeded.to_string(), result.to_string());
    }
}
