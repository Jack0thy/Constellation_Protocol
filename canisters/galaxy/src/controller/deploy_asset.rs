use crate::controller::types::{DeploymentResult, DeployAssetContractArgs};
use crate::ethereum::deploy_ethereum_contract::deploy_ethereum_contract;
use crate::ethereum::types::DeployEthereumContractArgs;
use ic_cdk::api::call::call;
use crate::controller::create_icrc3_canister::{create_icrc3_canister};
use crate::caller_is_controller;
use crate::controller::asset_mapping::add_asset_mapping;
use serde_json::Value;

// This is the main function that deploys an ICRC Canister and binds it to a deployed Ethereum contract
// Creates and ICRC3 canister, mints that canister_id onto an ERC1155 Ethereum contract, then initializes the ICRC canister with the Ethereum contract data
// Adds the asset mapping to the constellation and sets the contract address in the ICRC canister

#[ic_cdk::update(guard = "caller_is_controller")]
// init_chain_data is the data that will be used to initialize the ICRC canister, 
// Must be a json string, and can be managed by the frontend, set a form.  
pub async fn deploy_asset(args: DeployAssetContractArgs, init_chain_data: String) -> Result<DeploymentResult, String> {
    // Debug print before Ethereum call
    ic_cdk::println!("Starting deployment process...");

    // Create ICRC canister
    let icrc_canister_id = create_icrc3_canister().await;
    
    // Convert Principal to String for initial_uri
    let canister_uri = icrc_canister_id.to_text();

    // Currently taking the initial_uri from the args, but we will update this to use the metadata JSON on mainnet
    // args.initial_uri = canister_uri.clone() + ":4943/metadata"; // this won't work on mainnet, obviously

    // Call to Ethereum canister
    let ethereum_result: Result<String, String> = deploy_ethereum_contract(DeployEthereumContractArgs {
        initial_uri: args.initial_uri.clone(),
        name: args.name.clone(),
        symbol: args.symbol.clone(),
        canister_id: canister_uri.clone(),
    }).await;

    // Debug print after Ethereum call
    ic_cdk::println!("Ethereum call completed"); 
   

    // Add Eth Data to the ICRC Genesis Block (init_chain_data)
    let genesis_block_data = {
        // Parse the initial chain data
        let mut init_json: Value = serde_json::from_str(&init_chain_data)
            .map_err(|e| format!("Failed to parse init_chain_data: {}", e))?;
        
        // Parse the ethereum result
        let eth_json: Value = serde_json::from_str(&ethereum_result.clone().unwrap_or_default())
            .map_err(|e| format!("Failed to parse ethereum_result: {}", e))?;
        
        // Add ethereum data as a nested object under "eth_data" key
        if let Some(init_obj) = init_json.as_object_mut() {
            init_obj.insert("eth_data".to_string(), eth_json);
        }
        
        // Convert back to JSON string
        serde_json::to_string(&init_json)
            .map_err(|e| format!("Failed to serialize JSON with eth_data: {}", e))?
    };

    // Debug print before ICRC call
    ic_cdk::println!("Calling ICRC canister...");
    ic_cdk::println!("Genesis block data: {}", genesis_block_data);

    // Call to ICRC canister with the struct
    let icrc_result: Result<(String,), _> = call(
        icrc_canister_id,  
        "initialize_icrc3_ledger",  
        (genesis_block_data,)                          
    ).await;


    // TODO: On live canister, we need to set the metadata json here...this is just the basic premise for now

    // let set_metadata_call: Result<(String,), _> = call(
    //     icrc_canister_id,  // The canister to call
    //     "add_block",  
    //     (args.metadata_json.clone(), icrc_canister_id.to_text())  // Swap the order: (metadata, canister_id)
    // ).await;

    // // Handle the result of the set_metadata_call
    // match set_metadata_call {
    //     Ok(_) => ic_cdk::println!("Metadata set successfully."),
    //     Err(err) => return Err(format!("Failed to set metadata: {:?}", err)),
    // }   
    

    // Handle both results with more debug prints
    match (ethereum_result, icrc_result) {
        (Ok(eth_msg), Ok((_principal,))) => {
            ic_cdk::println!("✅ Ethereum deployment successful: {:?}", eth_msg);
            ic_cdk::println!("✅ ICRC deployment successful. New Principal: {}", icrc_canister_id.to_text());
            
            // Parse the JSON string to get contract address
            let contract_address = match serde_json::from_str::<Value>(&eth_msg) {
                Ok(json) => {
                    // Access the contract_address field directly
                    json["contract_address"]
                        .as_str()
                        .ok_or("Contract address not found in deployment result")?
                        .to_string()
                },
                Err(e) => return Err(format!("Failed to parse deployment result: {}", e)),
            };
            
            // Add the mapping with the extracted contract address
            add_asset_mapping(icrc_canister_id, contract_address.clone(), args).await?;

            // Add the contract address to the asset canister
            let add_contract_address_result: Result<(String,), _> = call(
                icrc_canister_id,  
                "set_contract_address",  
                (contract_address.clone(),)                          
            ).await;

            match add_contract_address_result {
                Ok(_) => ic_cdk::println!("Contract address set successfully."),
                Err(err) => ic_cdk::println!("Failed to set contract address: {:?}", err),
            }   

            
            Ok(DeploymentResult {
                eth_address: contract_address,
                canister_id: icrc_canister_id,
            })
        },
        (Err(msg), _) => Err(format!("Ethereum error: {}", msg)),
        (_, Err((_, msg))) => Err(format!("ICRC error: {}", msg.to_string())),
        // (Ok(Err(msg)), _) => Err(format!("Ethereum returned error: {}", msg.to_string())),
    }    
}