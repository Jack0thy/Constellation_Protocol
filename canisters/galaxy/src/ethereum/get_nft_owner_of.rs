use alloy::{
    transports::icp::IcpConfig,
    providers::ProviderBuilder,
    primitives::{Address, U256},
    sol,
};
use crate::{     
    RPC_SERVICE,
    controller::asset_mapping::get_asset_contract_address
};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ConstellationUpload,
    // CHANGED THE FOLLOWING TO WORK WITH THE ERC1155 CONTRACT, HAVEN'T TESTED JUST A PLACEHOLDER SO I COULD COMPILE
    "src/abi/AssetGovernance1155.json"
}

// Get the owner of an NFT
async fn get_balance_of_eth_call(contract_address: String, wallet_address: String, token_id: u64) -> Result<u64, String> {
    let contract_address = contract_address.parse::<Address>().unwrap();
    
    let wallet_address = wallet_address.parse::<Address>().unwrap();

    let token_id = U256::from(token_id);

    ic_cdk::println!("Checking balance of token ID {} in wallet {} for contract {}", token_id, wallet_address, contract_address);
    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    let contract = ConstellationUpload::new(contract_address, provider);
    let balance = contract.balanceOf(wallet_address, token_id).call().await;  

    match balance {
        Ok(balance) => {
            ic_cdk::println!("Successfully retrieved balance: {}", balance._0);
            Ok(balance._0.to_string().parse::<u64>().unwrap()) // Converts Uint256 to u64
        },
        Err(e) => {
            ic_cdk::println!("Error getting balance: {}", e);
            Err(e.to_string())
        }
    }
}

// balanceOf Service (manual args, for testing)
#[ic_cdk::update]
async fn get_balance_of(contract_address: String, wallet_address: String, token_id: u64) -> Result<u64, String> {    
    get_balance_of_eth_call(contract_address, wallet_address, token_id).await
}

// balanceOf Service (automatic contract address, for testing)
// #[ic_cdk::update(guard = "caller_is_asset")]
#[ic_cdk::update]
async fn get_balance_of_for_constellation_nft(wallet_address: String, token_id: u64) -> Result<u64, String> {
    ic_cdk::println!("Getting balance of for constellation principal: {}", ic_cdk::caller().to_text());
    let constellation_contract_address = get_asset_contract_address(ic_cdk::caller());
    match constellation_contract_address {
        Some(contract_address) => {
            get_balance_of_eth_call(contract_address, wallet_address, token_id).await
        },
        None => Err("Constellation not found".to_string())
    }
}