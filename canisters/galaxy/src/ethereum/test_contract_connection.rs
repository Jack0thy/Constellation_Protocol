use alloy::{
    primitives::{Address, utils::keccak256, Bytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::{request::TransactionRequest, BlockNumberOrTag},
    transports::icp::IcpConfig,
};
use crate::{caller_is_controller, RPC_SERVICE};

/// Simple test to verify connection to the smart contract
#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn test_contract_connection(contract_address: String) -> Result<String, String> {
    // Parse contract address
    let contract_address = contract_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid address: {}", e))?;

    // Setup basic provider
    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    // Create call to owner() function
    let owner_selector = keccak256(b"owner()")[..4].to_vec();
    
    // Create a simple call (not a transaction)
    let call = TransactionRequest {
        to: Some(contract_address.into()),
        input: owner_selector.into(),
        ..Default::default()
    };

    // Debug logging
    // ic_cdk::println!("Calling contract at: {}", contract_address);
    // ic_cdk::println!("TransactionRequest: {:?}", call);

    
    match provider.raw_request::<_, Bytes>(
        std::borrow::Cow::Borrowed("eth_call"),
        (call, BlockNumberOrTag::Latest)
    ).await {
        Ok(result) => {
            ic_cdk::println!("Raw response: {:?}", result);
            Ok(format!("Successfully connected! Response: {:?}", result))
        },
        Err(e) => {
            ic_cdk::println!("Error: {:?}", e);
            Err(format!("Failed to connect: {}", e))
        },
    }
}