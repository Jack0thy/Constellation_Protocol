use crate::ERC1155_CONTRACT_ADDRESS;
use crate::caller_is_controller;

#[ic_cdk::query(guard = "caller_is_controller")]
fn get_contract_address() -> String {
    let address = ERC1155_CONTRACT_ADDRESS.with(|address| address.borrow().get().to_string());
    ic_cdk::println!("Contract address: {}", address);
    address
}