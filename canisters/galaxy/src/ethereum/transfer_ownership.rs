// I don't think I ever got this working...

use crate::create_icp_signer;
use alloy::{
    primitives::Address,
    providers::ProviderBuilder,
    network::EthereumWallet,
    transports::icp::IcpConfig,
    sol,
};
use crate::ethereum::types::TransferOwnershipArgs;
use crate::{caller_is_controller, RPC_SERVICE};
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    AssetGovernance1155,
    "src/abi/AssetGovernance1155.json"
}

/// Transfers ownership of the contract to a new address
/// NOT TESTED

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn transfer_ownership(args: TransferOwnershipArgs) -> Result<String, String> {
    // Setup signer
    let signer = create_icp_signer().await;

    // Setup provider
    let wallet = EthereumWallet::from(signer);
    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_icp(config);

    // Parse addresses
    let contract_address = args.contract_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid contract address: {}", e))?;
    
    let new_owner = args.new_owner
        .parse::<Address>()
        .map_err(|e| format!("Invalid new owner address: {}", e))?;

    let contract = AssetGovernance1155::new(contract_address, provider);
    let _result = contract.transferOwnership(new_owner).send().await
        .map_err(|e| format!("Error: {}", e))?;

    Ok("Done".to_string())
} 