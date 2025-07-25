use crate::controller::types::{AddEthBlockArgs, AddBlockInfo, AddIcrcBlockArgs};
use ic_cdk::api::call::call;
use candid::{Principal};
use crate::ethereum::mint_token::mint_token;
use crate::ethereum::types::MintTokenArgs;
use crate::caller_is_controller;
use crate::controller::asset_mapping::get_asset_contract_address;

// TODO: Make a mapping that's accessible to both galaxy and constellation that maps a block_type to a token_id

// This is the main and general add_block function 
// Mints a token on Ethereum and adds a block to the ICRC canister simultaneously 
// There should be associated token_id's on ERC 1155 that match to specific block types on ICRC3
// This will allow us to track the token_id's on the Ethereum side and the block types on the ICRC side
// Principal is the ICRC canister ID, eth_args is the Ethereum args, and icrc_args 

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn add_block(principal: Principal, eth_args: AddEthBlockArgs, icrc_args: AddIcrcBlockArgs) -> Result<String, String> {
    // Call to Ethereum canister
    let ethereum_result: Result<String, String> = mint_token(MintTokenArgs{
        contract_address: get_asset_contract_address(principal).unwrap_or_else(|| String::from("0x3A43bFD1dCc4370C5d48d653c834d16e7E671313")),
        to: eth_args.eth_address.clone(),            
        content_hash: String::from("someHash"),  // TODO: Add the actual hash here once we decide what we're doing      
        token_id: Some(eth_args.token_id.clone()),
        amount: Some(eth_args.amount.clone()),        
        metadata_url: Some(eth_args.eth_metadata_url.clone()),
    }).await;

    // Call to ICRC canister with the struct
    // let asset_principal = Principal::from_text(ASSET_PRINCIPAL.with(|principal| principal.borrow().get().to_string()));
    let icrc_result: Result<(String,), _> = call(
        principal,
        "add_block",  
        (AddBlockInfo{
            kind: icrc_args.kind.clone(),
            block_data: icrc_args.block_data.clone(),            
        },)                                                      
    ).await;

    // Handle both results
    match (ethereum_result, icrc_result) {
        (Ok(eth_msg), Ok(_)) => Ok(eth_msg),
        (Err(e), _) => Err(format!("Ethereum error: {}", e)),
        (_, Err(e)) => Err(format!("ICRC error: {:?}", e)),
    }
}
