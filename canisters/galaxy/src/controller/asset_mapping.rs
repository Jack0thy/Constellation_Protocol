use candid::Principal;
use alloy::primitives::Address;
use std::vec::Vec;
use crate::caller_is_controller;
use hex;
use crate::{ASSET_MAPPINGS};
use crate::controller::types::{DeployAssetContractArgs, AssetMapping, AssetMappingDisplay, AssetMetadata};


// Asset mapping is the function that binds an ICRC canister ID to an Ethereum contract address
// Should see heavy use, first step in any query or update function so needs refinement and refactoring.
// It also stores the metadata for the asset
// Principal is the ICRC canister ID, eth_address is the Ethereum contract address, and args is the deployment args

/// Update function to add a new constellation mapping
/// Requires controller access and validates the Ethereum address
#[ic_cdk::update]
pub async fn add_asset_mapping(
    asset_id: Principal,
    eth_address: String,
    args: DeployAssetContractArgs,
) -> Result<AssetMapping, String> {
    // Only controllers can add mappings
    caller_is_controller()?;

    let contract_address = match eth_address.parse::<Address>() {
        Ok(address) => address,
        Err(e) => return Err(format!("Invalid Ethereum address: {}", e)),
    };

    // Creates basic metadata from deployment args (can be updated/upgraded later) ()
    let metadata = AssetMetadata {
        name: args.name,  
        symbol: args.symbol,                           
    };

    // Create and store the mapping
    let mapping = AssetMapping::new(
        asset_id,
        contract_address,
        metadata,
    );

    ASSET_MAPPINGS.with(|mappings| {
        mappings.borrow_mut().insert(asset_id, mapping.clone())
    });

    Ok(mapping)
}

/// Query function to get a constellation's contract address
/// Returns the address in human-readable hex format
#[ic_cdk::query]
pub fn get_asset_contract_address(canister_id: Principal) -> Option<String> {
    ASSET_MAPPINGS.with(|mappings| {
        mappings.borrow().get(&canister_id)
            .map(|mapping| format!("0x{}", hex::encode(&mapping.contract_address)))
    })
}

// Helper function to check if a mapping exists
#[ic_cdk::query]
pub fn has_asset_mapping(asset_id: Principal) -> bool {
    ASSET_MAPPINGS.with(|mappings| {
        mappings.borrow().contains_key(&asset_id)
    })
}

// Keep the get_all_mappings function which returns the display version
#[ic_cdk::query]
pub fn get_all_mappings() -> Vec<AssetMappingDisplay> {
    ASSET_MAPPINGS.with(|mappings| {
        mappings.borrow()
            .iter()
            .map(|(_, value)| AssetMappingDisplay {
                canister_id: value.canister_id,
                contract_address: format!("0x{}", hex::encode(&value.contract_address)),
                metadata: value.metadata.clone(),
            })
            .collect()
    })
}

// Get the metadata for an asset
#[ic_cdk::query]
pub fn get_asset_metadata(canister_id: Principal) -> Option<AssetMetadata> {
    ASSET_MAPPINGS.with(|mappings| {
        mappings.borrow().get(&canister_id).map(|mapping| mapping.metadata.clone())
    })
}

// Get the metadata for an asset as a JSON string
#[ic_cdk::query]
pub fn get_asset_metadata_json(canister_id: Principal) -> Option<String> {
    get_asset_metadata(canister_id).map(|metadata| serde_json::to_string(&metadata).unwrap_or_default())
}

// Get all the mappings as a JSON string
#[ic_cdk::query]
pub fn json_get_all_mappings() -> String {
    let mappings = get_all_mappings();
    serde_json::to_string(&mappings).unwrap_or_default()
}