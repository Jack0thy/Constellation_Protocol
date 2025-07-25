use ic_asset_certification::{Asset, AssetConfig, AssetEncoding, AssetFallbackConfig};
use ic_http_certification::{StatusCode, HttpRequest, HttpResponse};
use ic_cdk::{
    trap, 
    api::{set_certified_data, data_certificate}
};
use std::borrow::Cow;
use crate::ASSET_ROUTER;
use super::*;


// ## INDEX ASSET ##
// Finding and preparing the index asset
// fn prepare_index_asset<'a>() -> Asset<'a, 'a> {
//     let index_html= ASSETS_DIR.get_file("index.html").unwrap_or_else(|| 
//         trap("index.html not found")
//     ).contents();
    
//     Asset::new(
//         "index.html",
//         Cow::Borrowed(index_html)
//     )
// }

// fn prepare_index_asset_config() -> AssetConfig {
//     AssetConfig::File {
//         path: "index.html".to_string(), // Path to asset
//         content_type: Some("text/html".to_string()), // Content type
//         headers: vec![(
//             "cache-control".to_string(), // Cache control header
//             "public, no-cache, no-store".to_string() // Cache control value
//         )],
//         fallback_for: vec![AssetFallbackConfig {
//             scope: "/".to_string(), // Fallback scope (will return the index asset at root)
//             status_code: Some(StatusCode::OK), // Fallback status code
//         }],
//         aliased_by: vec!["/".to_string()], // Aliased by (will return the index asset at root)
//         encodings: vec![
//             AssetEncoding::Gzip.default_config() // Only gzip encoding is supported for now
//         ]
//     }
// }

// // ## GLOB ASSETS ##
// // Finding various assets based on glob patterns (e.g. images, fonts, etc.)
// fn prepare_glob_assets<'a>(glob: &str) -> Vec<Asset<'a, 'a>> {
//     let mut assets = Vec::new();

//     for entry in ASSETS_DIR
//         .find(glob)
//         .unwrap()
//         .map(|entry| entry.as_file().unwrap())  
//     {
//         let asset = Asset::new(
//             entry.path().to_str().unwrap(),
//             Cow::Borrowed(entry.contents())
//         );
//         assets.push(asset);
//     }
//     assets
// }

// fn prepare_glob_asset_config(glob: &str, content_type: &str) -> AssetConfig {
//     AssetConfig::Pattern {
//         pattern: glob.to_string(),
//         content_type: Some(content_type.to_string()),
//         headers: vec![(
//             "cache-control".to_string(),
//             "public, max-age=31536000, immutable".to_string()
//         )],
//         encodings: vec![
//             AssetEncoding::Gzip.custom_config(".gzip".to_string())
//         ],
//     }
// }

// // ## PREPARE NOT_FOUND ASSET ##
// fn prepare_not_found_asset<'a>() -> Asset<'a, 'a> {
//     Asset::new(
//         "404.html",
//         Cow::Borrowed("
//                     <!DOCTYPE html><html><head><title>404 Not Found</title></head><body><h1>404 - Page Not Found</h1></body></html>"
//                     .as_bytes()
//                 )
//     )
// }

// fn prepare_not_found_asset_config() -> AssetConfig {
//     AssetConfig::File {
//         path: "404.html".to_string(),
//         content_type: Some("text/html".to_string()),
//         headers: vec![
//             ("Cache-Control".to_string(), "public, no-cache, no-store".to_string()),
//         ],
//         fallback_for: vec![
//             AssetFallbackConfig {
//                 scope: "/css".to_string(),
//                 status_code: Some(StatusCode::NOT_FOUND),
//             },
//             AssetFallbackConfig {
//                 scope: "/js".to_string(),
//                 status_code: Some(StatusCode::NOT_FOUND),
//             },
//         ],
//         aliased_by: vec![
//             "/404".to_string(),
//             "/404/".to_string(),
//             "/404.html".to_string(),
//             "/not-found".to_string(),
//             "/not-found/".to_string(),
//             "/not-found/index.html".to_string(),
//     ],
//         encodings: vec![],
//     }
// }

// // ## CERTIFY ASSETS ##
// pub fn certify_assets(globs: &[AssetGlob]) {
//     let mut assets: Vec<Asset<'_, '_>> = Vec::new();
//     let mut asset_configs: Vec<AssetConfig> = Vec::new();
    
//     // Index asset
//     assets.push(prepare_index_asset());
//     asset_configs.push(prepare_index_asset_config());

//     // Glob assets
//     for glob in globs {
//         assets.extend(prepare_glob_assets(&glob.pattern));
//         asset_configs.push(prepare_glob_asset_config(&glob.pattern, &glob.content_type));
//     }

//     // Not found asset
//     assets.push(prepare_not_found_asset());
//     asset_configs.push(prepare_not_found_asset_config());

//     // Registering and certifying assets
//     ASSET_ROUTER.with_borrow_mut(|asset_router| {
//         asset_router.certify_assets(assets, asset_configs).unwrap();
//         set_certified_data(&asset_router.root_hash());
//     });
// }

// // ## SERVE ASSETS ##
// pub fn serve_certified_asset<'a>(req: &HttpRequest<'a>) -> HttpResponse<'a> {
//     let data_certificate = data_certificate().unwrap();
//     ASSET_ROUTER.with_borrow(|asset_router| {
//         asset_router.serve_asset(&data_certificate, req).unwrap()
//     })
// }