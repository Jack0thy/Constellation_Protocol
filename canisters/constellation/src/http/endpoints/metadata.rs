use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use crate::{
    PUBLIC_METADATA, PUBLIC_METADATA_PATH, PUBLIC_METADATA_TREE_PATH, PUBLIC_METADATA_CEL_EXPR_DEF, PUBLIC_METADATA_CEL_EXPR,
    http::response::{create_response, certify_response_only},
    RESPONSES
};
use super::*;

pub fn certify_public_metadata_response() {
    let mut response = create_public_metadata_response();
    certify_response_only(PUBLIC_METADATA_PATH, &mut response, &PUBLIC_METADATA_TREE_PATH, &PUBLIC_METADATA_CEL_EXPR_DEF);
}

pub fn create_public_metadata_response() -> HttpResponse<'static> {
    let body: String = PUBLIC_METADATA.with_borrow(|metadata|
        // Public metadata is stored as a String using JSON format
        metadata.get(&"current".to_string()).unwrap_or_default()
    );

    let additional_headers = vec![
        ("content-type".to_string(), "application/json".to_string()),
        ("content-length".to_string(), body.len().to_string()),
    ];

    create_response(StatusCode::OK, body.as_bytes().to_vec(), additional_headers, &PUBLIC_METADATA_CEL_EXPR)
}

pub fn public_metadata_handler(req: &HttpRequest<'_>, _params: &Params) -> HttpResponse<'static> {
    RESPONSES.with_borrow(|responses| {
        let path = extract_path_and_query(&req);
        let certified_response = responses.get(&path).unwrap();
        let mut response = certified_response.response.clone();
        let certificate = certified_response.certification.clone();

        add_ic_certificate_header(&req, &mut response, &PUBLIC_METADATA_TREE_PATH, &certificate);
        response
    })
}