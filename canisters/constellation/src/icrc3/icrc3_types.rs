use candid::CandidType;
use serde::{Deserialize, Serialize};
use icrc_ledger_types::icrc3::blocks::ICRC3GenericBlock;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use ic_stable_structures::Storable;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

// Add a struct to store the blockchain
#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<ICRC3GenericBlock>,
}

#[derive(CandidType)]
pub struct BlockType {
    pub block_type: String,
    pub url: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct BlockInfo {
    pub block_number: u128,
    pub transaction_type: String,  // "researcher_addition", "description_update", etc.
    pub timestamp: u64,
    // Could add more metadata fields if needed
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AddBlockInfo {
    pub kind: String,
    pub block_data: String,    
    // Could add more metadata fields if needed
}

// StableICRC3Value wrapper
#[derive(Clone, Debug)]
pub struct StableICRC3Value(pub ICRC3Value);

impl Deref for StableICRC3Value {
    type Target = ICRC3Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StableICRC3Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


// StableICRC3Value implementation (TODO: make a generic implementation for all types)
impl Storable for StableICRC3Value {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).expect("Encoding failed"))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(candid::decode_one(&bytes).expect("Decoding failed"))
    }

    const BOUND: ic_stable_structures::storable::Bound = 
        ic_stable_structures::storable::Bound::Unbounded;
}



