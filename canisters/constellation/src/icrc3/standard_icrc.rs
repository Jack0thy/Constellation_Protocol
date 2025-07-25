use ic_cdk::api::data_certificate;
use icrc_ledger_types::icrc3::archive::{GetArchivesArgs, GetArchivesResult};
use icrc_ledger_types::icrc3::blocks::DataCertificate;
use crate::BLOCKS;
use super::*;

#[ic_cdk::query]
fn icrc3_get_archives(_args: GetArchivesArgs) -> GetArchivesResult {
    ic_cdk::println!("Archive functionality not implemented yet");
    Vec::new()
}

#[ic_cdk::query]
fn icrc3_get_tip_certificate() -> Option<DataCertificate> {
    BLOCKS.with(|blocks| {
        let blocks = blocks.borrow();
        let last_index = (blocks.len() - 1) as u128;
        
        if let Some(StableICRC3Value(latest_block)) = blocks.get(&last_index) {
            let block_hash = calculate_block_hash(&latest_block);
            
            if let Some(certificate) = data_certificate() {
                return Some(DataCertificate {
                    certificate: Some(serde_bytes::ByteBuf::from(certificate)),
                    hash_tree: serde_bytes::ByteBuf::from(block_hash),
                });
            }
        }
        None
    })
}

#[ic_cdk::query]
fn icrc3_supported_block_types() -> Vec<BlockType> {
    vec![
        BlockType { block_type: "researcher_addition".to_string(), url: "https://example.com/docs".to_string() },
        BlockType { block_type: "mint".to_string(), url: "https://icrc3-standard/docs/mint".to_string() },
        BlockType { block_type: "transfer".to_string(), url: "https://icrc3-standard/docs/transfer".to_string() },
        BlockType { block_type: "burn".to_string(), url: "https://icrc3-standard/docs/burn".to_string() }        
    ]
}