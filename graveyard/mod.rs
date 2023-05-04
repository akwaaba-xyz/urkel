use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};
use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    types::Address,
};
use eyre::Result;
use std::sync::Arc;

struct Proof<'r>(&'r str);

#[derive(Debug)]
enum ProofError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Proof<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

async fn rust_inline_generation() -> Result<()> {
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