use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    types::Address,
};
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // The abigen! macro expands the contract's code in the current scope
    // so that you can interface your Rust program with the blockchain
    // counterpart of the contract.
    abigen!(
        SemaphoreVerifier,
        r#"[
            function verifyProof(uint256 merkleTreeRoot, uint256 nullifierHash, uint256 signal, uint256 externalNullifier, uint256[8] calldata proof, uint256 merkleTreeDepth) external view override
        ]"#,
    );

    const RPC_URL: &str = "https://arbitrum-mainnet.infura.io/v3/c747a474a81e4377a942bd8d051f5c24";
    const VERIFIER_ADDRESS: &str = "0xCAbeED6cB96a287000aBd834b0B79c05e6Ea4d07";

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let address: Address = VERIFIER_ADDRESS.parse()?;
    let contract = SemaphoreVerifier::new(address, client);

    if let Ok(verifyProof) = contract.verifyProof().call().await {
        println!("Proof is Ok!");
    } else {
        println!("Proof failed.");
    }

    Ok(())
}