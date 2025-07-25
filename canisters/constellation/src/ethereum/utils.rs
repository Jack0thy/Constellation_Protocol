use ic_cdk::call;
use candid::Principal;
use crate::config::CanisterIds;

pub async fn get_siwe_principal_eth_address(principal: Principal) -> Result<String, String> {
    let canister_ids = CanisterIds::current();
    let ic_siwe_provider = canister_ids.ic_siwe_provider;

    // Define the response type to match the Candid interface
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    enum GetAddressResponse {
        Ok(String),
        Err(String),
    }

    let eth_address: Result<(GetAddressResponse,), (ic_cdk::api::call::RejectionCode, String)> = call(
        ic_siwe_provider,
        "get_address",
        (principal.as_slice(),),
    )
    .await;

    match eth_address {
        Ok((response,)) => match response {
            GetAddressResponse::Ok(address) => Ok(address),
            GetAddressResponse::Err(msg) => Err(format!("SIWE provider error: {}", msg)),
        },
        Err((code, msg)) => Err(format!("Failed to get ETH address: {:?} - {}", code, msg))
    }
}

pub async fn get_balance_of(wallet_address: String, token_id: u64) -> Result<u64, String> {
    let canister_ids = CanisterIds::current();
    let galaxy_canister = canister_ids.galaxy;

    // Define the response type to match the Candid interface
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    enum GetBalanceResponse {
        Ok(u64),
        Err(String),
    }

    let balance: Result<(GetBalanceResponse,), (ic_cdk::api::call::RejectionCode, String)> = call(
        galaxy_canister,
        "get_balance_of_for_constellation_nft",
        (wallet_address, token_id),
    )
    .await;

    match balance {
        Ok((response,)) => match response {
            GetBalanceResponse::Ok(balance) => Ok(balance),
            GetBalanceResponse::Err(msg) => Err(format!("Galaxy error: {}", msg)),
        },
        Err((code, msg)) => Err(format!("Failed to get balance: {:?} - {}", code, msg))
    }
}

pub async fn is_token_owner(wallet_address: String, token_id: u64) -> Result<bool, String> {
    let balance = get_balance_of(wallet_address, token_id).await.unwrap_or_else(|e| {
        ic_cdk::println!("Error: {:?}", e);
        0 // Return 0 if there's an error, which will evaluate to false
    });
    ic_cdk::println!("Balance: {:?}", balance);
    Ok(balance > 0)
}