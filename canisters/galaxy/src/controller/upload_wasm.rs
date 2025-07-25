use crate::caller_is_controller;
use std::cell::RefCell;
use std::collections::HashMap;
use candid::Principal;

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks

#[derive(Default)]
struct WasmBytes {
    chunks: Vec<Vec<u8>>,
    total_size: usize,
}

thread_local! {
    static WASM_BYTES: RefCell<HashMap<Principal, WasmBytes>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update(guard = "caller_is_controller")]
pub fn upload_wasm(wasm_bytes: Vec<u8>) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    // Check if wasm_bytes is empty
    if wasm_bytes.is_empty() {
        return Err("WASM bytes cannot be empty".to_string());
    }
    
    // Split the wasm_bytes into 1MB chunks
    let mut chunks = Vec::new();
    let mut offset = 0;
    
    while offset < wasm_bytes.len() {
        let end = if offset + CHUNK_SIZE > wasm_bytes.len() {
            wasm_bytes.len()
        } else {
            offset + CHUNK_SIZE
        };
        let chunk = wasm_bytes[offset..end].to_vec();
        chunks.push(chunk);
        offset = end;
    }
    
    // Store the chunks separately to avoid memory issues
    WASM_BYTES.with(|m| {
        m.borrow_mut().insert(caller, WasmBytes {
            chunks,
            total_size: wasm_bytes.len(),
        });
    });
    
    Ok(())
}

#[ic_cdk::query]
pub fn get_stored_wasm() -> Option<Vec<u8>> {
    let caller = ic_cdk::caller();
    WASM_BYTES.with(|m| {
        m.borrow().get(&caller).map(|wasm| {
            // Combine chunks back into a single Vec<u8>
            let mut combined = Vec::with_capacity(wasm.total_size);
            for chunk in &wasm.chunks {
                combined.extend(chunk);
            }
            combined
        })
    })
}