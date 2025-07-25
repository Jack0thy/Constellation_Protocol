use ic_http_certification::{HttpRequest, HttpResponse};
use ic_cdk::api::data_certificate;
use crate::{
    QUERY_ROUTER, ASSET_ROUTER, UPDATE_ROUTER,
    http::{
        types::RouteHandler,
        endpoints::{            
            metadata::public_metadata_handler
        },
        utils::extract_path_and_query
    }
};

// Route Preparation

pub fn prepare_query_handler() {   
    insert_query_route("GET", "/metadata", public_metadata_handler);
}

pub fn prepare_update_handler() {   
}

// Route Insertion

fn insert_query_route(method: &str, path: &str, route_handler: RouteHandler) {
    QUERY_ROUTER.with_borrow_mut(|query_router| {
        let router = query_router.entry(method.to_string()).or_default();

        router.insert(path, route_handler).unwrap();
    });
}

fn insert_update_route(method: &str, path: &str, route_handler: RouteHandler) {
    UPDATE_ROUTER.with_borrow_mut(|update_router| {
        let router = update_router.entry(method.to_string()).or_default();
        router.insert(path, route_handler).unwrap();
    });
}

// Route Matching

fn match_asset_route<'a>(req: &HttpRequest<'a>) -> HttpResponse<'static> {
    ASSET_ROUTER.with_borrow(|asset_router| {
        let cert = data_certificate().unwrap();
        let data_certificate = cert.as_slice();
        asset_router.serve_asset(data_certificate, &req).unwrap()
    })
}

pub fn match_route<'a>(req: &HttpRequest<'a>) -> HttpResponse<'static> {
    let path = extract_path_and_query(req);

    QUERY_ROUTER.with_borrow(move |query_router| {
        let method_router = query_router
            .get(&req.method().as_str().to_uppercase());
        
        match method_router {
            None => {
                ic_cdk::println!("No route found for method: {}", req.method());
                match_asset_route(&req)},
            Some(router) => match router.at(&path) {
                Ok(handler_match) => {
                    ic_cdk::println!("Route found for method: {}", req.method());
                    let handler = handler_match.value;
                    let params = handler_match.params.clone();
                    handler(&req, &params)
                }
                Err(_) => {
                    ic_cdk::println!("No route found for path: {}", path);
                    match_asset_route(&req)
                }
            }
        }
    })
}