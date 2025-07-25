use crate::{create_icp_signer, caller_is_controller, RPC_SERVICE};
use alloy::{
    primitives::{Address, U256, Bytes},
    providers::ProviderBuilder,
    network::EthereumWallet,
    transports::icp::IcpConfig,
    // signers::Signer,
    sol,
};
use crate::ethereum::types::MintTokenArgs;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    AssetGovernance1155,
    "src/abi/AssetGovernance1155.json"
}

// Mints general token(s) on the Ethereum blockchain (with ERC1155 can mint multiple tokens at once)
// It is guarded by the caller_is_controller function 
// It is an async function that returns a Result<String, String>
// The Result<String, String> is a tuple of the result and an error message
// The result is a string that is the transaction hash
// The error message is a string

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn mint_token(args: MintTokenArgs) -> Result<String, String> {
    // Setup signer
    let signer = create_icp_signer().await;
    // let address = signer.address();

    // Setup canister wallet
    let wallet = EthereumWallet::from(signer);
   
    // Setup provider
    let rpc_service = RPC_SERVICE.clone();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_icp(config);

    // Contract address
    let contract_address = args.contract_address.parse::<Address>()
        .map_err(|e| format!("Error: {}", e))?;

    // Initialize contract instance
    let contract = AssetGovernance1155::new(contract_address, provider);

    // Call the function and get the transaction hash
    match contract.mintToken(
        args.to.parse::<Address>().unwrap(),
        args.token_id.unwrap().parse::<U256>().unwrap(),
        args.amount.unwrap().parse::<U256>().unwrap(),
        args.metadata_url.unwrap(),
        args.content_hash,
        Bytes::from(String::from("0x").into_bytes()),
    ).send().await {
        Ok(tx) => {
            let tx_hash = *tx.tx_hash();
            Ok(format!("{:?}", tx_hash))
        },
        Err(e) => Err(format!("Error: {}", e))
    }
}