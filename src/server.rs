use tonic::{transport::Server, Request, Response, Status};

use campusing::document_service_server::{DocumentService, DocumentServiceServer};
use campusing::{CreateDocumentRequest, Document};

pub mod campusing {
    tonic::include_proto!("campusing.common.v1"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct DocumentServiceImpl {}

#[tonic::async_trait]
impl DocumentService for DocumentServiceImpl {
    async fn create_document(
        &self,
        request: Request<CreateDocumentRequest>,
    ) -> Result<Response<Document>, Status> {
        println!("Got a request: {:?}", request);

        let response = Document {
            content: "".to_string(),
            path: None,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let document_server = DocumentServiceImpl::default();

    Server::builder()
        .add_service(DocumentServiceServer::new(document_server))
        .serve(addr)
        .await?;

    Ok(())
}
