use candid::CandidType;
use serde::{Deserialize, Serialize};
    
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct DeployEthereumContractArgs {
    pub name: String,
    pub symbol: String,
    pub initial_uri: String,
    pub canister_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MintTokenArgs {
    pub contract_address: String,
    pub to: String,
    pub token_id: Option<String>,
    pub amount: Option<String>,    
    pub content_hash: String,
    pub metadata_url: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MintUniqueTokenArgs {
    pub contract_address: String,
    pub to: String,
    pub token_id: String,
    pub metadata_url: String,
    pub content_hash: String,
}



#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RegisterPublicNFTArgs {
    pub contract_address: String,
    pub token_id: u64,
    pub eth_contract_address: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct TransferOwnershipArgs {
    pub contract_address: String,
    pub new_owner: String,
}




