use candid::{Encode, Principal};
use ic_cdk::api::management_canister::{
    main::{
        CanisterInstallMode, CreateCanisterArgument, InstallChunkedCodeArgument,
        UploadChunkArgument, create_canister, install_chunked_code, upload_chunk,
    }    
};
use sha2::{Sha256, Digest};
use crate::caller_is_controller;
use crate::controller::upload_wasm::get_stored_wasm;

// This is the main function that creates an ICRC 3 canister from the galaxy 
// It is used to create the ICRC 3 canister for the constellation
// It is guarded by the caller_is_controller function 
// It is an async function that returns the Principal of the created canister (which is the canister ID)

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn create_icrc3_canister() -> Principal {
    let caller = ic_cdk::caller();

    // Create the canister (all these values can be changed later)
    let principal = match create_canister(
        CreateCanisterArgument {
            // settings: Some(CanisterSettings {
            //     // TODO: Remove the hardcoded controller once we have a proper controller
            //     controllers: Some(vec![
            //         ic_cdk::id(), 
            //         caller.clone(), 
            //         Principal::from_text("nllbr-e4sfh-tjf54-ltw5f-p6ic2-ohjz7-sykq7-aop2t-v7mjt-zzolf-jae").unwrap()]), // TODO: Remove this once we have a proper controller
            //     compute_allocation: None,
            //     memory_allocation: None,
            //     freezing_threshold: None,
            //     log_visibility: None,
            //     reserved_cycles_limit: None,
            //     wasm_memory_limit: None,
            // }),
            
            // Playground compatibility
            settings: None,
        },
        1_000_000_000_000,
    )
    .await
    {
        Err((code, msg)) => ic_cdk::trap(&format!("Rejection Code: {:?}, Message: {:?}", code, msg)),
        Ok((principal,)) => principal.canister_id,
    };

    // This isn't working properly yet, memory isn't stable. 
    // Get the WASM bytes from storage
    let wasm_module = match get_stored_wasm() { // This isn't working properly yet, memory isn't stable. 
        Some(wasm) => wasm,
        None => ic_cdk::trap("No WASM module stored. Please upload WASM first using upload_wasm()"),
    };

    // Calculate WASM hash
    let mut hasher = Sha256::new();
    hasher.update(&wasm_module);
    let wasm_hash = hasher.finalize().to_vec();

    // Store upload results
    let mut chunk_hashes = Vec::new();

    // Upload WASM in chunks
    let mut offset = 0;
    while offset < wasm_module.len() {
        let end = std::cmp::min(offset + CHUNK_SIZE, wasm_module.len());
        let chunk = &wasm_module[offset..end];
        
        let upload_result = upload_chunk(
            UploadChunkArgument {
                canister_id: principal,
                chunk: chunk.to_vec(),
            },
        ).await;

        match upload_result {
            Ok((hash,)) => {
                chunk_hashes.push(hash);
                offset = end;
            }
            Err((code, msg)) => ic_cdk::trap(&format!("Failed to upload chunk: {:?}, {}", code, msg)),
        }
    }

    // Prepare init args (empty in this case)
    let init_args = Encode!().unwrap();

    // Install chunked WASM
    let install_result = install_chunked_code(
        InstallChunkedCodeArgument {
            mode: CanisterInstallMode::Install,
            target_canister: principal,
            store_canister: Some(principal),  
            chunk_hashes_list: chunk_hashes,
            wasm_module_hash: wasm_hash,
            arg: init_args,
        }
    )
    .await;

    match install_result {
        Ok(_) => principal,
        Err((code, msg)) => ic_cdk::trap(&format!("Failed to install code: {:?}, {}", code, msg)),
    }
}
