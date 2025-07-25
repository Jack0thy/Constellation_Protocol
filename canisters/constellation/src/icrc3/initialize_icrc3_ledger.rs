use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use ic_cdk::api::time;
use std::collections::BTreeMap;
use crate::BLOCKS;  
use super::*;
use crate::caller_is_controller;

#[ic_cdk::update(guard = "caller_is_controller")]
fn initialize_icrc3_ledger(init_data: String) -> String {
    BLOCKS.with(|blocks| {
        if !blocks.borrow().is_empty() {
            return "Error: Blockchain already initialized".to_string();
        }

        // Create transaction structure (consistent with regular blocks)
        let mut transaction_map: BTreeMap<String, ICRC3Value> = BTreeMap::new();
        transaction_map.insert("kind".to_string(), ICRC3Value::Text("genesis".to_string()));
        transaction_map.insert("timestamp".to_string(), ICRC3Value::Nat((time() / 1_000_000_000).into()));

        // Add init data to transaction (consistent structure)
        let mut transaction_data: BTreeMap<String, ICRC3Value> = BTreeMap::new();
        transaction_data.insert("init_data".to_string(), ICRC3Value::Text(init_data.clone()));
        transaction_map.insert("data".to_string(), ICRC3Value::Map(transaction_data));

        // Create block data
        let block_data = vec![
            ("phash".to_string(), ICRC3Value::Blob(serde_bytes::ByteBuf::from(vec![]))),
            ("timestamp".to_string(), ICRC3Value::Nat(time().into())),
            ("transaction".to_string(), ICRC3Value::Map(transaction_map)),
            ("btype".to_string(), ICRC3Value::Text("genesis".to_string())),
        ]
        .into_iter()
        .collect();

        // Genesis block
        let genesis_block = StableICRC3Value(ICRC3Value::Map(block_data));

        // Store in stable memory
        blocks.borrow_mut().insert(0, genesis_block);

        "Ledger initialized successfully".to_string()
    })
}
