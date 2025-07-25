// Required imports for Ethereum address handling and stable storage
use alloy::primitives::Address;
use ic_stable_structures::Storable;
use std::{borrow::Cow, ops::Deref};

/// Wrapper struct around an Ethereum address that implements stable storage
#[derive(Clone, Debug)]
pub struct StableAddress(pub Address);

/// Implements Deref to allow transparent access to the underlying Address
impl Deref for StableAddress {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implementation of Storable trait to allow StableAddress to be stored in stable memory
impl Storable for StableAddress {
    /// Converts the address to a byte array for storage
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(self.into_array().to_vec())
    }

    /// Reconstructs an address from stored bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Address::from_slice(&bytes))
    }

    /// Ethereum addresses are always exactly 20 bytes
    const BOUND: ic_stable_structures::storable::Bound = 
        ic_stable_structures::storable::Bound::Bounded { max_size: 20, is_fixed_size: true };
}
