use candid::Principal;
use ic_cdk::api::management_canister::main::delete_canister;
use crate::caller_is_controller;
use ic_cdk::api::management_canister::main::{ CanisterIdRecord, stop_canister };

// This is the main function that deletes an ICRC 3 canister from the galaxy
// Doesn't burn the ethereum contract at this point, but will in the future
// Mostly used for testing purposes and so galaxy doesn't get clogged up with canisters

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn delete_constellation_canister(canister_id: Principal) -> Result<String, String> {
    // Verify the canister exists and caller has permission
    if !ic_cdk::api::is_controller(&ic_cdk::caller()) {
        return Err("Caller is not a controller of the canister".to_string());
    }

    // Stop the canister first
    match stop_canister(CanisterIdRecord { canister_id }).await {
        Ok(_) => {
            // Delete the canister
            match delete_canister(CanisterIdRecord { canister_id }).await {
                Ok(_) => {
                    // Remove from mappings
                    crate::ASSET_MAPPINGS.with(|mappings| {
                        mappings.borrow_mut().remove(&canister_id);
                    });
                    Ok("Constellation Canister Deleted".to_string())
                },
                Err((code, msg)) => Err(format!("Failed to delete canister: {:?}, {}", code, msg)),
            }
        },
        Err((code, msg)) => Err(format!("Failed to stop canister: {:?}, {}", code, msg)),
    }
}