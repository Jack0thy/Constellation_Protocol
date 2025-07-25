use ic_http_certification::{
    HttpRequest, HttpResponse, HttpCertificationPath, HttpCertificationTreeEntry, HttpCertification,
    utils::add_v2_certificate_header
};
use ic_cdk::api::data_certificate;
use crate::HTTP_TREE;

pub fn extract_path_and_query(req: &HttpRequest) -> String {
    let mut path = req.get_path().expect("Failed to get path").to_string();
    if req.get_query().is_ok() {
        let query = req.get_query().unwrap();
        if query.is_some() {
            path = format!("{}?{}", path, query.unwrap())
        }
    }
    path
}

pub fn add_ic_certificate_header(request: &HttpRequest, response: &mut HttpResponse<'static>, tree_path: &HttpCertificationPath, certificate: &HttpCertification) {
    let req_path = request.get_path().expect("Failed to get request path");

    HTTP_TREE.with(|http_tree| {
        add_v2_certificate_header(
            &data_certificate().expect("Failed to get data certificate"),
            response,
            &http_tree.borrow().witness(
                &HttpCertificationTreeEntry::new(tree_path, certificate),
                &req_path
            ).unwrap(),
            &tree_path.to_expr_path()
        )
    })
}