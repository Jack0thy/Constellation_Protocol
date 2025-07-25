use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use crate::BLOCKS;
use super::*;

#[ic_cdk::query]
fn verify_chain_integrity() -> bool {
    BLOCKS.with(|blocks| {
        let blocks = blocks.borrow();
        let len = blocks.len();
        
        // Check each block
        for i in 1..len {
            if let Some(StableICRC3Value(ICRC3Value::Map(block_data))) = blocks.get(&(i as u128)) {
                if let Some(ICRC3Value::Blob(stored_phash)) = block_data.get("phash") {
                    if let Some(StableICRC3Value(prev_block)) = blocks.get(&((i - 1) as u128)) {
                        let calculated_prev_hash = calculate_block_hash(&prev_block);
                        
                        if stored_phash != &serde_bytes::ByteBuf::from(calculated_prev_hash) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    })
}