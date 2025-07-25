use sha2::{Sha256, Digest};
use serde_bytes;
use super::*;

//**ICRC3 HELPER FUNCTIONS**
// Helper function to calculate block hash according to ICRC-3 standard
pub fn hash_value(value: &ICRC3Value) -> Vec<u8> {
    let mut hasher = Sha256::new();
    
    match value {
        ICRC3Value::Nat(n) => {
            // Simple encoding for Nat - we can improve this later if needed
            hasher.update(n.to_string().as_bytes());
        },
        ICRC3Value::Text(text) => {
            // UTF-8 encoding for Text
            hasher.update(text.as_bytes());
        },
        ICRC3Value::Blob(blob) => {
            // Direct bytes for Blob
            hasher.update(blob);
        },
        ICRC3Value::Map(map) => {
            // For Map type, sort keys first for consistent ordering
            let mut sorted_keys: Vec<_> = map.keys().collect();
            sorted_keys.sort();
            
            for key in sorted_keys {
                if let Some(value) = map.get(key) {
                    // Hash key
                    hasher.update(key.as_bytes());
                    // Hash value recursively
                    let value_hash = hash_value(value);
                    hasher.update(&value_hash);
                }
            }
        },
        ICRC3Value::Array(arr) => {
            // For arrays, hash each element in order
            for value in arr {
                let element_hash = hash_value(value);
                hasher.update(&element_hash);
            }
        },
        _ => {
            // For other types, use their string representation
            hasher.update(format!("{:?}", value).as_bytes());
        }
    }
    
    hasher.finalize().to_vec()
}

// Calculate block hash using the hash_value function
pub fn calculate_block_hash(block: &ICRC3Value) -> Vec<u8> {
    hash_value(block)
}

// Calculate previous hash
pub fn calculate_previous_hash(blockchain: &Blockchain) -> serde_bytes::ByteBuf {
    blockchain
        .blocks
        .last()
        .map(|block| serde_bytes::ByteBuf::from(calculate_block_hash(block)))
        .unwrap_or_else(|| serde_bytes::ByteBuf::from(vec![]))
}

// Helper function to safely extract text from ICRC3Value
pub fn extract_text(value: Option<&ICRC3Value>) -> String {
    match value {
        Some(ICRC3Value::Text(text)) => text.clone(),
        _ => String::new()
    }
}