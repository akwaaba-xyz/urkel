// gRPC
pub mod openfga {
    tonic::include_proto!("openfga.v1");
}
use openfga::{open_fga_service_client::OpenFgaServiceClient, GetStoreRequest};

pub async fn get_store() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OpenFgaServiceClient::connect("http://[::1]:8081").await?;

    let request = tonic::Request::new(GetStoreRequest {
        store_id: "01GXKJ136DJQ2AESP71PWR9M7P".into(),
    });

    let response = client.get_store(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}