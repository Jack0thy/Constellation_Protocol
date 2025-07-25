use alloy::{
    network::TxSigner,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::icp::IcpConfig,
};
use crate::{caller_is_controller, RPC_SERVICE, create_icp_signer};

// This is the function that gets the balance of the galaxy wallet
#[ic_cdk::update(guard = "caller_is_controller")]
async fn get_balance(address: Option<String>) -> Result<String, String> {
    let address = match address {
        Some(val) => val,
        None => {
            let signer = create_icp_signer().await;
            signer.address().to_string()
        }
    };
    let address = address.parse::<Address>().map_err(|e| e.to_string())?;
    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);
    let result = provider.get_balance(address).await;

    match result {
        Ok(balance) => Ok(balance.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
