use crate::{
    ethereum::utils::{get_siwe_principal_eth_address, get_balance_of}
};

// Caller should be a SIWE principal
#[ic_cdk::update]
async fn is_token_owner_service(token_id: u64) -> bool {
    let eth_address = get_siwe_principal_eth_address(ic_cdk::caller()).await.unwrap_or_else(|e| {
        ic_cdk::println!("Error: {:?}", e);
        "".to_string()
    });
    let balance = get_balance_of(eth_address, token_id).await.unwrap_or_else(|e| {
        ic_cdk::println!("Error: {:?}", e);
        0 // Return 0 if there's an error, which will evaluate to false
    });
    ic_cdk::println!("Balance: {:?}", balance);
    balance > 0
}