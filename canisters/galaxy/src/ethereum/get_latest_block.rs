use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::icp::IcpConfig,
};
use crate::caller_is_controller;
use crate::RPC_SERVICE;

// Get the latest block number from the Ethereum blockchain
// Basic test function to see if we can connect to the Ethereum blockchain

#[ic_cdk::update(guard = "caller_is_controller")]
async fn get_latest_block() -> Result<String, String> {
    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);
    let result = provider.get_block_number().await;

    match result {
        Ok(block) => Ok(block.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
