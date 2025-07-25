use ic_http_certification::{
    StatusCode, HttpResponse, HttpCertification, HttpCertificationTreeEntry, HttpCertificationPath,
    CERTIFICATE_EXPRESSION_HEADER_NAME, HttpRequest, DefaultFullCelExpression, DefaultResponseOnlyCelExpression
};
use ic_cdk::api::set_certified_data;
use crate::{
    HTTP_TREE, RESPONSES,
    http::utils::extract_path_and_query
};
use super::types::*;

pub fn create_response(
    status_code: StatusCode,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
    cel_expr: &str
) -> HttpResponse<'static> {
    HttpResponse::builder()
        .with_status_code(status_code)
        .with_headers({
            let mut h = headers;
            h.extend(vec![
                (
                    CERTIFICATE_EXPRESSION_HEADER_NAME.to_string(),
                    cel_expr.to_string(),
                ),
                (
                    "strict-transport-security".to_string(),
                    "max-age=31536000; includeSubDomains".to_string(),
                ),
                ("x-content-type-options".to_string(), "nosniff".to_string()),
                ("referrer-policy".to_string(), "no-referrer".to_string()),
                (
                    "cache-control".to_string(),
                    "no-store, max-age=0".to_string(),
                ),
                ("pragma".to_string(), "no-cache".to_string()),
            ]);
            h
        })
        .with_body(body)
        .build()
}

pub fn certify_full(
    request: HttpRequest,
    response: &mut HttpResponse<'static>,
    tree_path: &HttpCertificationPath,
    cel_expr_def: &DefaultFullCelExpression,
    key: Option<String>
) {
    let key = if key.is_none() {
        extract_path_and_query(&request)
    } else {
        key.unwrap()
    };

    // retrieve and remove any existing response for the request method and path
    let existing_response = RESPONSES.with_borrow_mut(|responses| {
        responses.remove(&key)
    });

    // if there is an existing response, remove its certification from the certification tree
    if let Some(existing_response) = existing_response {
        HTTP_TREE.with(|http_tree| {
            http_tree.borrow_mut().delete(&HttpCertificationTreeEntry::new(
                tree_path.clone(),
                &existing_response.certification,
            ));
        })
    }

    // create the certification for this response and CEL expression pair
    let certification =
        HttpCertification::full(cel_expr_def, &request, &response, None).unwrap_or_else(
            |e| {
                ic_cdk::println!("Error certifying response: {:?}", e);
                HttpCertification::full(cel_expr_def, &request, &response, None).unwrap()
            }
        );

    ic_cdk::println!("Certifying full response for key: {}", key);

    RESPONSES.with_borrow_mut(|responses| {
        // store the response for later retrieval
        responses.insert(
            key,
            CertifiedHttpResponse {
                response: response.clone(),
                certification: certification.clone(),
            },
        );
    });

    HTTP_TREE.with(|http_tree| {
        // insert the certification into the certification tree
        http_tree.borrow_mut().insert(&HttpCertificationTreeEntry::new(tree_path, &certification));

        // set the canister's certified data
        set_certified_data(&http_tree.borrow().root_hash());
    });
}

pub fn certify_response_only(
    path: &'static str,
    response: &mut HttpResponse<'static>,
    tree_path: &HttpCertificationPath,
    cel_expr_def: &DefaultResponseOnlyCelExpression,
) {
    let path = path.to_string();
    // retrieve and remove any existing response for the request method and path
    let existing_response = RESPONSES.with_borrow_mut(|responses| {
        responses.remove(&path)
    });

    // if there is an existing response, remove its certification from the certification tree
    if let Some(existing_response) = existing_response {
        HTTP_TREE.with(|http_tree| {
            http_tree.borrow_mut().delete(&HttpCertificationTreeEntry::new(
                tree_path.clone(),
                &existing_response.certification,
            ));
        })
    }

    // create the certification for this response and CEL expression pair
    let certification =
        HttpCertification::response_only(cel_expr_def, &response, None).unwrap_or_else(
            |e| {
                ic_cdk::println!("Error certifying response: {:?}", e);
                HttpCertification::response_only(cel_expr_def, &response, None).unwrap()
            }
        );

    RESPONSES.with_borrow_mut(|responses| {
        // store the response for later retrieval
        responses.insert(
            path,
            CertifiedHttpResponse {
                response: response.clone(),
                certification: certification.clone(),
            },
        );
    });

    HTTP_TREE.with(|http_tree| {
        // insert the certification into the certification tree
        http_tree.borrow_mut().insert(&HttpCertificationTreeEntry::new(tree_path, &certification));

        // set the canister's certified data
        set_certified_data(&http_tree.borrow().root_hash());
    });    
}