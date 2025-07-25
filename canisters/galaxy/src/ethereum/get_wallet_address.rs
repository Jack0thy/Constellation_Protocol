use crate::create_icp_signer;
use alloy::signers::Signer;
use crate::caller_is_controller;

/// Get the wallet address of the wallet that is connected to the galazy canister.
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn get_address() -> Result<String, String> {
    let signer = create_icp_signer().await;
    let address = signer.address();
    Ok(address.to_string())
}
