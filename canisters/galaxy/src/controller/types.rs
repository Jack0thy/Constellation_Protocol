use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use alloy::primitives::Address;
use ic_stable_structures::Storable;
use std::borrow::Cow;


#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct AddEthBlockArgs {
    pub eth_address: String,
    pub token_id: String,
    pub amount: String,    
    pub eth_metadata_url: String,   
}

// This is the args for the ICRC canister
// kind is the type of block, block_data is the data for the block (typically a json string, can have as much data as needed)
#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct AddIcrcBlockArgs {
    pub kind: String,
    pub block_data: String,       
}


#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct DeployAssetContractArgs {
    pub name: String,
    pub symbol: String,
    // keeping initial_uri for now in case we want to demo with IPFS, but it will be updated to the metadata JSON
    pub initial_uri: String, 
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct EthereumContractArgs {
    pub name: String,
    pub symbol: String,
    pub initial_uri: String,
    pub canister_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct MintArgs {
    pub contract_address: String,
    pub to: String,
    pub content_hash: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct AddBlockInfo {
    pub kind: String,
    pub block_data: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct DeploymentResult {
    pub eth_address: String,
    pub canister_id: Principal,
}

// this is the metadata that will be stored in the ICRC canister
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AssetMetadata {
    pub name: String,
    pub symbol: String,    
}

/// Main structure that links a constellation's canister ID with its blockchain contract
/// and associated metadata
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct AssetMapping {
    pub canister_id: Principal,     // ICP canister identifier
    pub contract_address: Vec<u8>,  // Ethereum contract address as bytes
    pub metadata: AssetMetadata, // Associated constellation information
}

// Define a new struct for displaying the mapping
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AssetMappingDisplay {
    pub canister_id: Principal,
    pub contract_address: String,  // Now as hex string
    pub metadata: AssetMetadata,  // Add metadata field
}

impl AssetMapping {
    /// Creates a new ConstellationMapping instance
    /// Converts the Ethereum address to bytes for storage
    pub fn new(canister_id: Principal, contract_address: Address, metadata: AssetMetadata) -> Self {
        Self {
            canister_id,
            contract_address: contract_address.to_vec(),
            metadata,
        }
    }

    /// Retrieves the Ethereum address from stored bytes
    pub fn get_address(&self) -> Address {
        Address::from_slice(&self.contract_address)
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct WasmBytes(pub Vec<u8>);


impl Storable for WasmBytes {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(bytes.to_vec())
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded { max_size: 1024 * 1024, is_fixed_size: false };
}


/// Implementation for stable storage of AssetMapping
/// This probably isn't the best implementation, but it works for now
impl Storable for AssetMapping {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];
        
        // Store Principal length and bytes
        // We prefix with length to handle variable-sized principals
        let principal_bytes = self.canister_id.as_slice();
        bytes.extend_from_slice(&(principal_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(principal_bytes);
        
        // Store contract address length and bytes
        // Standard Ethereum addresses are 20 bytes
        bytes.extend_from_slice(&(self.contract_address.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&self.contract_address);
        
        // Store metadata using Candid serialization
        // This handles all the string fields in the metadata struct
        let metadata_bytes = candid::encode_one(&self.metadata)
            .expect("Failed to serialize metadata");
        bytes.extend_from_slice(&(metadata_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&metadata_bytes);
        
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let bytes = bytes.as_ref();
        let mut pos = 0;

        // Read Principal data
        let principal_len = u32::from_be_bytes(bytes[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let principal_bytes = &bytes[pos..pos+principal_len];
        pos += principal_len;
        
        // Read contract address
        let addr_len = u32::from_be_bytes(bytes[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let address_bytes = &bytes[pos..pos+addr_len];
        pos += addr_len;

        // Read and deserialize metadata
        let metadata_len = u32::from_be_bytes(bytes[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let metadata_bytes = &bytes[pos..pos+metadata_len];
        
        // Try to deserialize with new format first, fall back to old format
        let metadata: AssetMetadata = match candid::decode_one::<AssetMetadata>(metadata_bytes) {
            Ok(metadata) => metadata,
            Err(_) => {
                // Try old format with just a metadata field
                #[derive(CandidType, Deserialize)]
                struct OldAssetMetadata {
                    metadata: String,
                }
                
                match candid::decode_one::<OldAssetMetadata>(metadata_bytes) {
                    Ok(old_metadata) => {
                        // Convert old format to new format with default values
                        AssetMetadata {
                            name: "Legacy Asset".to_string(),
                            symbol: "Unknown".to_string(),
                        }
                    },
                    Err(e) => {
                        // If both fail, create a default metadata
                        ic_cdk::println!("Failed to deserialize metadata: {:?}", e);
                        AssetMetadata {
                            name: "Unknown Asset".to_string(),
                            symbol: "Unknown".to_string(),
                        }
                    }
                }
            }
        };

        Self {
            canister_id: Principal::from_slice(principal_bytes),
            contract_address: address_bytes.to_vec(),
            metadata,
        }
    }

    // Define storage bounds for the stable memory
    const BOUND: ic_stable_structures::storable::Bound = 
        ic_stable_structures::storable::Bound::Bounded { 
            max_size: 4096,      // Maximum bytes for one mapping (4KB)
            is_fixed_size: false // Variable size due to strings in metadata
        };
}

