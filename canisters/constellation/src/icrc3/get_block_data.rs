use crate::{BLOCKS, HASH_INDEX, BLOCK_TYPE_INDEX, BLOCK_TYPE_TO_NUMBERS, StableVec};
use super::*;

/// Retrieves a block by its index number from the blockchain
#[ic_cdk::query]
pub fn get_block(index: u128) -> Option<ICRC3Value> {
    BLOCKS.with(|blocks| {
        blocks.borrow().get(&(index as u128))
            .map(|StableICRC3Value(block)| block)
    })
}

/// Retrieves a block by index and returns it as a JSON string
#[ic_cdk::query]
pub fn json_get_block(index: u128) -> String {
    get_json_string(&get_block(index).unwrap())
}

/// Retrieves a block by its hash value, with hash verification
#[ic_cdk::query]
fn get_block_by_hash(hash: String) -> Option<ICRC3Value> {
    let target_hash = match hex::decode(hash) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };
    
    // Look up block number in hash index
    HASH_INDEX.with(|index| {
        let block = index.borrow()
            .get(&target_hash)
            .and_then(get_block);

        let block_hash = calculate_block_hash(block.as_ref().unwrap());
        
        if block_hash == target_hash {
            block
        } else {
            None
        }
    })
}

/// Retrieves a block by hash and returns it as a JSON string
#[ic_cdk::query]
fn json_get_block_by_hash(hash: String) -> String {
    get_json_string(&get_block_by_hash(hash).unwrap())
}

/// Retrieves the genesis (first) block of the blockchain
#[ic_cdk::query]
pub fn get_genesis_block() -> Option<ICRC3Value> {
    get_block(0)
}

/// Retrieves the genesis block and returns it as a JSON string
#[ic_cdk::query]
fn json_get_genesis_block() -> String {
    get_json_string(&get_genesis_block().unwrap())
}

/// Retrieves the most recent blocks from the blockchain (up to specified count)
#[ic_cdk::query]
fn get_latest_blocks(count: u128) -> Vec<ICRC3Value> {
    BLOCKS.with(|blocks| {
        let blocks = blocks.borrow();
        let total_blocks = blocks.len() as u128;
        
        let start_index = if count >= total_blocks {
            0
        } else {
            total_blocks - count
        };
        
        let mut result = Vec::new();
        for i in start_index..total_blocks {
            if let Some(StableICRC3Value(block)) = blocks.get(&(i as u128)) {
                result.push(block);
            }
        }
        result
    })
}

/// Retrieves the latest blocks and returns them as a JSON string
#[ic_cdk::query]
fn json_get_latest_blocks(count: u128) -> String {
    get_json_string_from_vec(&get_latest_blocks(count))
}

/// Retrieves all blocks of a specific type efficiently using the reverse index
#[ic_cdk::query]
fn get_blocks_by_type(block_type: String) -> Vec<ICRC3Value> {
    let mut result = Vec::new();
    
    // Use the reverse index for O(1) lookup of block numbers by type
    BLOCK_TYPE_TO_NUMBERS.with(|reverse_index| {
        let reverse_index = reverse_index.borrow();
        if let Some(StableVec(block_numbers)) = reverse_index.get(&block_type) {
            BLOCKS.with(|blocks| {
                let blocks = blocks.borrow();
                
                // Get blocks by their numbers
                for block_number in block_numbers {
                    if let Some(StableICRC3Value(block)) = blocks.get(&block_number) {
                        result.push(block);
                    }
                }
            });
        }
    });
    
    result
}

/// Retrieves blocks by type and returns them as a JSON string
#[ic_cdk::query]
fn json_get_blocks_by_type(block_type: String) -> String {
    get_json_string_from_vec(&get_blocks_by_type(block_type))
}

/// Gets all unique block types in the chain
#[ic_cdk::query]
fn get_all_block_types() -> Vec<String> {
    let mut block_types = Vec::new();
    
    BLOCK_TYPE_TO_NUMBERS.with(|reverse_index| {
        let reverse_index = reverse_index.borrow();
        for (block_type, _) in reverse_index.iter() {
            block_types.push(block_type.clone());
        }
    });
    
    block_types
}

/// Gets the count of blocks for each block type
#[ic_cdk::query]
fn get_block_type_counts() -> Vec<(String, u64)> {
    let mut counts = Vec::new();
    
    BLOCK_TYPE_TO_NUMBERS.with(|reverse_index| {
        let reverse_index = reverse_index.borrow();
        for (block_type, StableVec(block_numbers)) in reverse_index.iter() {
            counts.push((block_type.clone(), block_numbers.len() as u64));
        }
    });
    
    counts
}

