use crate::ERC1155_CONTRACT_ADDRESS;
use crate::ethereum::types::StableAddress;
use alloy::primitives::Address;
use std::str::FromStr;
use crate::caller_is_controller;

#[ic_cdk::update(guard = "caller_is_controller")]
fn set_contract_address(contract_address: String) -> String {
    ERC1155_CONTRACT_ADDRESS.with(|address| {
        address.borrow_mut()
            .set(StableAddress(Address::from_str(&contract_address).unwrap()))
            .expect("Failed to set the contract address");
    });
    "Contract address set successfully".to_string()
}   