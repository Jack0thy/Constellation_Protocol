pub mod ethereum;
pub mod controller;

use alloy::{
    signers::icp::IcpSigner,
    transports::icp::{RpcApi, RpcService},
};
use ic_cdk::export_candid;
pub use ethereum::types::{
    MintTokenArgs, 
    DeployEthereumContractArgs,    
    RegisterPublicNFTArgs, 
    TransferOwnershipArgs, 
    MintUniqueTokenArgs
};
use candid::Principal;
use crate::controller::types::*;
use std::{
    cell::RefCell,
    vec::Vec,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap
};
use once_cell::sync::Lazy;

//Quick switch between chains
const CHAIN_ID: u64 = 84532;
static RPC_SERVICE: Lazy<RpcService> = Lazy::new(get_rpc_service_base_sepolia);

// RPC Service Functions (for now, we are using the Base Sepolia testnet)

fn _get_rpc_service_kaleido() -> RpcService {
    RpcService::Custom(RpcApi {
        // for mainnet
        // url: "https://nginx-dj6cf.kinsta.app/%22".to_string(),
        // headers: None,
        // for testnet
        url: "https://eth-sepolia.g.alchemy.com/v2/wzyeihWQoIuQAaMZxJU5foFmYPKQ6oBg".to_string(),
        headers: None
    })
}

fn get_rpc_service_base_sepolia() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "https://base-sepolia.g.alchemy.com/v2/wzyeihWQoIuQAaMZxJU5foFmYPKQ6oBg".to_string(),
        headers: None
    })
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    

    static ASSET_MAPPINGS: RefCell<StableBTreeMap<Principal, AssetMapping, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9)))
        )
    );

    static WASM_BYTES: RefCell<StableBTreeMap<Principal, WasmBytes, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        )
    );

}

fn _get_rpc_service_base() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "https://ic-alloy-evm-rpc-proxy.kristofer-977.workers.dev/base-mainnet".to_string(),
        headers: None,
    })
}

fn _get_rpc_service_sepolia() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "http://localhost:8787/eth-sepolia".to_string(),
        headers: None,
    })
}

// ICP Thresold Cryptography Functions
fn get_ecdsa_key_name() -> String {
    #[allow(clippy::option_env_unwrap)]
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    match dfx_network {
        "local" => "dfx_test_key".to_string(),
        "ic" => "key_1".to_string(),
        _ => panic!("Unsupported network."),
    }
}

async fn create_icp_signer() -> IcpSigner {
    let ecdsa_key_name = get_ecdsa_key_name();
    // Try creating signer without chainId first
    match IcpSigner::new(vec![], &ecdsa_key_name, None).await {
        Ok(signer) => {
            ic_cdk::println!("Created signer without chainId");
            signer
        },
        Err(_) => {
            // Fallback to using chainId
            ic_cdk::println!("Attempting to create signer with chainId {}", CHAIN_ID);
            IcpSigner::new(vec![], &ecdsa_key_name, Some(CHAIN_ID))
                .await
                .unwrap_or_else(|e| panic!("Failed to create signer: {:?}", e))
        }
    }
}

//inline improves efficiency
#[inline(always)]
pub fn caller_is_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        // Err("Unauthorized access attempt".to_string())
        Ok(())  //toggle on and off for testing
    }
}

// Make sure this is the only export_candid! in the entire project
export_candid!();
