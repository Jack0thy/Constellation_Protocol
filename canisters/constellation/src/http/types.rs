use ic_http_certification::{HttpCertification, HttpResponse, HttpRequest};
use matchit::Params;
use serde::Serialize;

#[derive(Clone)]
pub struct CertifiedHttpResponse<'a> {
    pub response: HttpResponse<'a>,
    pub certification: HttpCertification,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetGlob<'a> {
    pub pattern: &'a str,
    pub content_type: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub enum ApiResponse<'a, T = ()> {
    #[serde(rename = "ok")]
    Ok { data: &'a T },
    #[serde(rename = "err")]
    Err { code: u16, message: String },
}

impl<'a, T: Serialize> ApiResponse<'a, T> {
    pub fn ok(data: &'a T) -> ApiResponse<T> {
        Self::Ok { data }
    }

    fn _err(code: u16, message: String) -> Self {
        Self::Err { code, message }
    }

    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize value")
    }
}

pub type ErrorResponse<'a> = ApiResponse<'a, ()>;


pub type RouteHandler = for<'a> fn(&'a HttpRequest, &'a Params) -> HttpResponse<'static>;