pub mod icrc3;
pub mod http;
pub mod services;
pub mod ethereum;
pub mod config;

// Third-party imports
use ic_cdk::{init, post_upgrade, query, export_candid};
use ic_http_certification::{DefaultCelBuilder, DefaultFullCelExpression, DefaultResponseOnlyCelExpression, DefaultResponseCertification, HttpCertificationPath, HttpCertificationTree, HttpRequest, HttpResponse};
use ic_asset_certification::AssetRouter;
use include_dir::{include_dir, Dir};
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    vec::Vec,
    time::Duration,
    borrow::Cow,
    ops::{Deref, DerefMut},
};
use icrc_ledger_types::{
    icrc::generic_value::ICRC3Value,
    icrc3::blocks::DataCertificate,
    icrc3::archive::{GetArchivesArgs, GetArchivesResult},
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell, Storable
};
use rand::{rngs::StdRng, SeedableRng, RngCore};
use getrandom::register_custom_getrandom;
use alloy::primitives::Address;
use matchit::Router;
use lazy_static::lazy_static;
// Internal imports

use crate::icrc3::icrc3_types::{    
    StableICRC3Value, 
    AddBlockInfo,
    BlockType, 
};

// Custom Storable wrapper for Vec<u128>
#[derive(Clone, Debug)]
pub struct StableVec<T>(pub Vec<T>);

impl<T> Deref for StableVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StableVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Storable for StableVec<u128> {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).expect("Encoding failed"))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(candid::decode_one(&bytes).expect("Decoding failed"))
    }

    const BOUND: ic_stable_structures::storable::Bound = 
        ic_stable_structures::storable::Bound::Unbounded;
}
use crate::http::{
    types::{CertifiedHttpResponse, RouteHandler, AssetGlob},
    // assets::certify_assets,
    router::{match_route, prepare_query_handler},
};

use crate::ethereum::{
    types::StableAddress,
    utils::{is_token_owner, get_siwe_principal_eth_address}
};
// Public methods
#[init]
fn init() {
    init_rng();
    // certify_assets(&ASSET_GLOBS);
    prepare_query_handler();
}

#[post_upgrade]
fn post_upgrade() {
    init();
}

#[query]
async fn http_request(req: HttpRequest<'_>) -> HttpResponse<'static> {
    ic_cdk::println!("HTTP request received: {:?}", req);
    let response = match_route(&req);
    ic_cdk::println!("HTTP response: {:?}", response);
    response
}

thread_local! {
    // getrandom fix: https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-limitations
    static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
    
    // HTTP Tree Certification
    static HTTP_TREE: Rc<RefCell<HttpCertificationTree>> = Rc::new(RefCell::new(HttpCertificationTree::default()));

    // Resonses
    static RESPONSES: RefCell<HashMap<String, CertifiedHttpResponse<'static>>> = RefCell::new(HashMap::new());
   

    // Routers
    static QUERY_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
    static UPDATE_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
    static ASSET_ROUTER: RefCell<AssetRouter<'static>> = HTTP_TREE.with(|http_tree| RefCell::new(AssetRouter::with_tree(http_tree.clone())));
    
    // Memory manager
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // Stable storage (persists across upgrades)
    static BLOCKS: RefCell<StableBTreeMap<u128, StableICRC3Value, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
    
    static HASH_INDEX: RefCell<StableBTreeMap<Vec<u8>, u128, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );   
   
    

    static ERC1155_CONTRACT_ADDRESS: RefCell<StableCell<StableAddress, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9))),
            StableAddress(Address::ZERO)
        ).expect("Failed to intialize the ERC1155 contract address cell")
    );

   



    static PUBLIC_METADATA: RefCell<StableBTreeMap<String, String, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(17)))
        )
    );

  
  
    // Block number to block type index for efficient querying
    static BLOCK_TYPE_INDEX: RefCell<StableBTreeMap<u128, String, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(19)))
        )
    );

    // Block type to block numbers index for reverse lookups
    static BLOCK_TYPE_TO_NUMBERS: RefCell<StableBTreeMap<String, StableVec<u128>, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(20)))
        )
    );
}

// Static assets
static ASSET_GLOBS: &[AssetGlob<'static>] = &[
    AssetGlob {
        pattern: "**/*.js*",
        content_type: "text/javascript",
    },
    AssetGlob {
        pattern: "**/*.css",
        content_type: "text/css",
    },
    AssetGlob {
        pattern: "**/*.png",
        content_type: "image/png",
    },
    AssetGlob {
        pattern: "**/*.webp",
        content_type: "image/webp",
    },
];

// Accessible Paths
const PUBLIC_METADATA_PATH: &str = "/metadata";

// Response Headers
const _STANDARD_RESPONSE_HEADERS: &[&str] = &[
    "strict-transport-security",
    "x-content-type-options",
    "referrer-policy",
    "cache-control",
    "pragma",
];



lazy_static! {
    // HTTP Tree Paths
   
    static ref PUBLIC_METADATA_TREE_PATH: HttpCertificationPath<'static> =
        HttpCertificationPath::exact(PUBLIC_METADATA_PATH);

    // CEL Expressions
    

    static ref PUBLIC_METADATA_CEL_EXPR_DEF: DefaultResponseOnlyCelExpression<'static> = 
        DefaultCelBuilder::response_only_certification()
            .with_response_certification(DefaultResponseCertification::certified_response_headers(&[
                "content-type",
                "content-length",
                "strict-transport-security",
                "x-content-type-options",
                "referrer-policy",
                "cache-control",
                "pragma",
            ]))
            .build();
    static ref PUBLIC_METADATA_CEL_EXPR: String = PUBLIC_METADATA_CEL_EXPR_DEF.to_string();
}

// getrandom fix: https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-limitations
// Randomness on the IC is async, and the custom getrandom function can't be async,
// so we seed an RNG instead of always calling raw_rand directly
fn init_rng() {
    // raw_rand is technically an inter-canister call, and you can't make those from lifecycle functions like #[init],
    // so we schedule an immediate timer to make the call instead
    ic_cdk_timers::set_timer(Duration::ZERO, || ic_cdk::spawn(async {
        let (seed,) = ic_cdk::api::management_canister::main::raw_rand().await.unwrap();
                // StdRng is from the `rand` crate. It makes for a good default but any RNG implementation would work
        RNG.with(|rng| *rng.borrow_mut() = Some(StdRng::from_seed(seed.try_into().unwrap())));
    }));
}
register_custom_getrandom!(custom_getrandom);
fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(buf));
    Ok(())
}


//***TOGGLE THIS TO ENABLE CONTROLLER CHECK***
#[inline(always)] //inline improves efficiency
pub fn caller_is_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        Ok(())        
        // Err("Unauthorized access attempt".to_string())
    }
}

pub async fn caller_is_token_owner(token_id: u64) -> bool {
    let wallet_address = get_siwe_principal_eth_address(ic_cdk::caller()).await.unwrap_or_else(|e| {
        e
    });

    let is_token_ower = is_token_owner(wallet_address, token_id).await.unwrap_or_else(|e| {
        ic_cdk::println!("Error checking if caller is token owner: {}", e);
        false
    });

    is_token_ower
}

export_candid!();