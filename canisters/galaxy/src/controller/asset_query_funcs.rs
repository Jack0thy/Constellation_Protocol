use crate::caller_is_controller;
use ic_cdk::api::call::call;
use candid::{Principal, CandidType};

// Series of functions that allow for querying the ICRC canister for data
// These are all update functions because they are making canister calls
// They are all currently guarded by the caller_is_controller function (will be removed later)
// They are all async functions that return a Result<String, String>
// The Result<String, String> is a tuple of the result and an error message
// The result is a JSON string


// Helper function to make canister calls
async fn call_asset_function<T: CandidType>(principal: Principal, method: &str, args: Option<T>) -> Result<String, String> {

    let icrc_result: Result<(String,), _> = match args {
        Some(arg) => call(principal, method, (arg,)).await,
        None => call(principal, method, ()).await,
    };

    match icrc_result {
        Ok(result) => Ok(result.0),
        Err(e) => Err(format!("Error calling {}: {:?}", method, e)),
    }
}

// Query functions (have to be update functions because they are making canister calls)
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn get_genesis_block(principal: Principal) -> Result<String, String> {
    call_asset_function::<()>(principal, "json_get_genesis_block", None).await
}

// Get the genesis block for an asset
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn get_block(principal: Principal, block_number: String) -> Result<String, String> {
    call_asset_function::<String>(principal, "json_get_block", Some(block_number)).await
}

// Get the entire chain for an asset
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn get_entire_chain(principal: Principal) -> Result<String, String> {
    call_asset_function::<()>(principal, "json_get_entire_chain", None).await
} 

// Get a block by hash for an asset
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn get_block_by_hash(principal: Principal, hash: String) -> Result<String, String> {
    call_asset_function::<String>(principal, "json_get_block_by_hash", Some(hash)).await
}

// Get a metadata value for an asset
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn get_metadata_value(principal: Principal, key: String) -> Result<String, String> {
    call_asset_function::<String>(principal, "json_get_metadata_value", Some(key)).await
}

