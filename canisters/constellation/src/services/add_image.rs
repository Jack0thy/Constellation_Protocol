use ic_asset_certification::{Asset, AssetConfig};
use crate::ASSET_ROUTER;

#[ic_cdk::update]
pub fn add_image(
    image_name: String,
    image_type: String,
    image_data: Vec<u8>,
) -> Result<String, String> {
    // Log incoming request
    ic_cdk::println!("Attempting to upload image of type: {}", image_type);
    ic_cdk::println!("Image data size: {} bytes", image_data.len());

    // Validate image type
    let content_type = match image_type.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        
        _ => {
            ic_cdk::println!("Invalid image type: {}", image_type);
            return Err("Unsupported image type. Supported types are: png, jpg, jpeg, gif, webp".to_string())
        }
    };

    // Validate image data
    if image_data.is_empty() {
        ic_cdk::println!("Empty image data received");
        return Err("Image data cannot be empty".to_string());
    }

    // Create asset with unique filename
    let filename = format!("{}.{}", image_name, image_type);
    ic_cdk::println!("Creating asset with filename: {}", filename);
    
    let asset = vec![Asset::new(filename.clone(), image_data)];

    // Configure asset with proper content type
    let asset_config = vec![AssetConfig::Pattern {
        pattern: format!("**/*.{}", image_type),
        content_type: Some(content_type.to_string()),
        headers: vec![(
            "cache-control".to_string(),
            "public, max-age=31536000, immutable".to_string()
        )],
        encodings: vec![],
    }];

    // Certify and store the asset
    ASSET_ROUTER.with_borrow_mut(|router| {
        match router.certify_assets(asset, asset_config) {
            Ok(_) => {
                ic_cdk::println!("Successfully certified asset: {}", filename);
                ic_cdk::api::set_certified_data(&router.root_hash());
                Ok(format!("Image {} uploaded successfully", filename))
            },
            Err(e) => {
                ic_cdk::println!("Failed to certify asset: {}", e);
                Err(format!("Failed to certify asset: {}", e))
            }
        }
    })
} 