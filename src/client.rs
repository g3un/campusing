use campusing::document_service_client::DocumentServiceClient;
use campusing::CreateDocumentRequest;

pub mod campusing {
    tonic::include_proto!("campusing.common.v1"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DocumentServiceClient::connect("http://[::1]:50051").await?;

    let req = tonic::Request::new(CreateDocumentRequest {});
    let res = client.create_document(req).await?;

    println!("{:?}", res);

    Ok(())
}
