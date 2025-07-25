use crate::{create_icp_signer, caller_is_controller, RPC_SERVICE};
use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    network::EthereumWallet,
    transports::icp::IcpConfig,
    // signers::Signer,
    sol,
};
use crate::ethereum::types::MintUniqueTokenArgs;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    AssetGovernance1155,
    "src/abi/AssetGovernance1155.json"
}

// Mints a unique token on the Ethereum blockchain 
// (the asset contract has provisions to ensure it's an NFT)

#[ic_cdk::update(guard = "caller_is_controller")]
pub async fn mint_unique_token(args: MintUniqueTokenArgs) -> Result<String, String> {
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
    // let contract_address = ERC1155_CONTRACT_ADDRESS.with(|address| address.borrow().get().to_string());
    // let contract_address = contract_address.parse::<Address>()
    //     .map_err(|e| format!("Error: {}", e))?;

    let contract_address = args.contract_address.parse::<Address>()
        .map_err(|e| format!("Error: {}", e))?;
    
    // Initialize contract instance
    let contract = AssetGovernance1155::new(contract_address, provider);

    // Call the function and get the transaction hash
    match contract.mintUniqueToken(
        args.to.parse::<Address>().unwrap(),
        args.token_id.parse::<U256>().unwrap(),
        args.metadata_url,
        args.content_hash,
    ).send().await {
        Ok(tx) => {
            let tx_hash = *tx.tx_hash();
            Ok(format!("{:?}", tx_hash))
        },
        Err(e) => Err(format!("Error: {}", e))
    }
}