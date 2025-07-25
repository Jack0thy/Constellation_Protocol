use candid::Principal;

// This file contains canister IDs for different environments
// It should be updated during the build process to reflect the current canister IDs

pub struct CanisterIds {
    pub ic_siwe_provider: Principal,
    pub galaxy: Principal,
}

impl CanisterIds {
    pub fn local() -> Self {
        Self {
            ic_siwe_provider: Principal::from_text("u6s2n-gx777-77774-qaaba-cai")
                .expect("Invalid ic_siwe_provider principal"),
            galaxy: Principal::from_text("gdh4h-lyaaa-aaaal-ar2ba-cai")
                .expect("Invalid galaxy principal"),
        }
    }

    pub fn production() -> Self {
        Self {
            ic_siwe_provider: Principal::from_text("cpmcr-yeaaa-aaaaa-qaala-cai")
                .expect("Invalid ic_siwe_provider principal"),
            galaxy: Principal::from_text("gdh4h-lyaaa-aaaal-ar2ba-cai")
                .expect("Invalid galaxy principal"),
        }
    }

    // For now, we'll use local IDs for development
    // In the future, this could be determined by environment variables or other means
    pub fn current() -> Self {
        Self::local()
    }
}
