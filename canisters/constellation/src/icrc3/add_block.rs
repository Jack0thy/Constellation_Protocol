use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::icrc3::transactions::Transaction;
use ic_cdk::api::time;
use std::collections::BTreeMap;
use hex;
use crate::{BLOCKS, HASH_INDEX, BLOCK_TYPE_INDEX, BLOCK_TYPE_TO_NUMBERS, StableVec};
use super::*;
use crate::caller_is_controller;

#[ic_cdk::update(guard = "caller_is_controller")]
fn add_block(
    block_info: AddBlockInfo  // Generic store identifier, requires stable store to be implemented
) -> String {
    BLOCKS.with(|blocks| {
        let mut blocks = blocks.borrow_mut();
        let current_index = blocks.len() as u128;

        let mut block_map: BTreeMap<String, ICRC3Value> = BTreeMap::new();
        
        // Convert Ethereum timestamp (assumed in seconds)
        let ic_timestamp = time() / 1_000_000_000;  // Convert nanoseconds to seconds

        // Create transaction structure
        let transaction = Transaction {
            kind: block_info.kind.clone(),
            mint: None,
            burn: None,
            transfer: None,
            approve: None,
            timestamp: ic_timestamp,
        };

        // Convert Transaction to ICRC3Value
        let mut transaction_map: BTreeMap<String, ICRC3Value> = BTreeMap::new();
        transaction_map.insert("kind".to_string(), ICRC3Value::Text(transaction.kind));
        transaction_map.insert("timestamp".to_string(), ICRC3Value::Nat(transaction.timestamp.into()));

        // Add block data to transaction
        let mut transaction_data: BTreeMap<String, ICRC3Value> = BTreeMap::new();
        transaction_data.insert("block_data".to_string(), ICRC3Value::Text(block_info.block_data.clone()));        
        transaction_map.insert("data".to_string(), ICRC3Value::Map(transaction_data));

        // Get previous block hash
        let prev_hash = if current_index > 0 {
            if let Some(StableICRC3Value(prev_block)) = blocks.get(&(current_index - 1)) {
                calculate_block_hash(&prev_block)
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        // Create block with appropriate transaction structure
        block_map.insert("phash".to_string(), ICRC3Value::Blob(serde_bytes::ByteBuf::from(prev_hash)));
        block_map.insert("timestamp".to_string(), ICRC3Value::Nat(time().into()));
        block_map.insert("transaction".to_string(), ICRC3Value::Map(transaction_map));
        block_map.insert("btype".to_string(), ICRC3Value::Text(block_info.kind.clone()));

        let new_block = ICRC3Value::Map(block_map);
        
        // Calculate transaction hash
        let transaction_hash = calculate_block_hash(&new_block);

        // Store the block
        blocks.insert(current_index, StableICRC3Value(new_block));

        // Store hash index
        HASH_INDEX.with(|index| {
            index.borrow_mut().insert(transaction_hash.clone(), current_index);
        });  
    
        ic_cdk::println!("Data: {}", block_info.block_data);        
     
        // Store block number to block type mapping for efficient querying
        BLOCK_TYPE_INDEX.with(|index| {
            index.borrow_mut().insert(current_index, block_info.kind.clone());
        });

        // Update reverse index: block type to block numbers
        BLOCK_TYPE_TO_NUMBERS.with(|reverse_index| {
            let mut reverse_index = reverse_index.borrow_mut();
            let existing_numbers = reverse_index.get(&block_info.kind.clone());
            let mut new_numbers = match existing_numbers {
                Some(StableVec(numbers)) => numbers.clone(),
                None => Vec::new(),
            };
            new_numbers.push(current_index);
            reverse_index.insert(block_info.kind.clone(), StableVec(new_numbers));
        });

        // Return the transaction hash
        format!("Block Addition recorded successfully. ICRC3 transaction hash: {}", hex::encode(transaction_hash))
    })
}



