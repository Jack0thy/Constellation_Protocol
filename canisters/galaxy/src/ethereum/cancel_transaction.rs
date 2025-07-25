use candid::CandidType;
use serde::{Deserialize, Serialize};
use alloy::{
    network::{EthereumWallet, TransactionBuilder, TxSigner}, primitives::U256, providers::{Provider, ProviderBuilder}, rpc::types::request::TransactionRequest, transports::icp::IcpConfig
};
use crate::{create_icp_signer, caller_is_controller, CHAIN_ID, RPC_SERVICE};

// Not really needed for base, but useful for testing on mainnet ethereum where things get bogged down

#[derive(CandidType, Serialize, Deserialize)]
pub struct CancelTransactionArgs {
    pub nonce: u64,
    pub priority_fee: u64,
    pub max_fee: u64,
}

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn cancel_transaction(nonce: Option<u64>) -> Result<String, String> {
    // Setup signer and provider
    let signer = create_icp_signer().await;
    let address = signer.address();
    let wallet = EthereumWallet::from(signer);

    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_icp(config);

    // **Fetch current nonce dynamically**
    let current_nonce = provider.get_transaction_count(address).await.unwrap_or(0);
    let final_nonce = nonce.unwrap_or(current_nonce); // Use given nonce or latest

    // **Fetch the current gas price from network**
    let base_gas_price = provider.get_gas_price().await.unwrap_or(800_000_000); // Default fallback
    let max_fee_per_gas = base_gas_price * 2;  // Increase by 100%
    let priority_fee = base_gas_price / 2;  // Increase by 50%

    ic_cdk::println!("Gas price: {}, Priority Fee: {}, Max Fee: {}", base_gas_price, priority_fee, max_fee_per_gas);

    // Create cancellation transaction
    let cancel_tx = TransactionRequest::default()
        .with_from(address)
        .with_to(address)  // Send to self
        .with_value(U256::from(0))  // 0 ETH transfer
        .with_chain_id(CHAIN_ID)  // Sepolia chain ID
        .with_nonce(final_nonce)  // Use latest nonce or provided one
        .with_gas_limit(21_000)  // Standard transfer gas limit
        .with_max_priority_fee_per_gas(priority_fee)
        .with_max_fee_per_gas(max_fee_per_gas);

    // Build and send transaction
    match cancel_tx.build(&wallet).await {
        Ok(tx_envelope) => {
            match provider.send_tx_envelope(tx_envelope).await {
                Ok(pending_tx) => {
                    let tx_hash = *pending_tx.tx_hash();
                    Ok(format!("Cancellation transaction sent. Hash: {:?}", tx_hash))
                },
                Err(e) => Err(format!("Failed to send cancellation transaction: {:?}", e))
            }
        },
        Err(e) => Err(format!("Failed to build cancellation transaction: {:?}", e))
    }
}
