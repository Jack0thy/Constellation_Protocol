use crate::{
    ethereum::utils::get_siwe_principal_eth_address
};

#[ic_cdk::update]
async fn get_siwe_principal_eth_address_service() -> String {
    let principal = ic_cdk::caller();
    ic_cdk::println!("Principal: {:?}", principal);
    let eth_address = get_siwe_principal_eth_address(principal).await.unwrap_or_else(|e| {
        ic_cdk::println!("Error: {:?}", e);
        "".to_string()
    });
    ic_cdk::println!("Eth address: {:?}", eth_address);
    eth_address
}