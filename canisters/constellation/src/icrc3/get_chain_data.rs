use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use crate::BLOCKS;
use super::*;

/// Retrieves the complete blockchain as a vector of all blocks
#[ic_cdk::query]
fn get_entire_chain() -> Vec<ICRC3Value> {
    BLOCKS.with(|blocks| {
        let blocks = blocks.borrow();
        let mut chain = Vec::new();
        for i in 0..blocks.len() {
            if let Some(StableICRC3Value(block)) = blocks.get(&(i as u128)) {
                chain.push(block);
            }
        }
        chain
    })
}

/// Retrieves the entire blockchain and returns it as a JSON string
#[ic_cdk::query]
fn json_get_entire_chain() -> String {
    get_json_string_from_vec(&get_entire_chain())
}

/// Returns basic information about the blockchain (block count, first/last block status)
#[ic_cdk::query]
fn chain_info() -> String {
    BLOCKS.with(|blocks| {
        let blocks = blocks.borrow();
        format!("Number of blocks: {}\nFirst block exists: {}\nLast block exists: {}",
            blocks.len(),
            blocks.get(&0).is_some(),
            blocks.get(&((blocks.len() - 1) as u128)).is_some()
        )
    })
}

/// Returns the total number of blocks in the blockchain
#[ic_cdk::query]
fn get_chain_length() -> u64 {
    BLOCKS.with(|blocks| blocks.borrow().len() as u64)
}

