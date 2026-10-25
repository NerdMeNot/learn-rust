use greeter::pb::greeter_server::{Greeter, GreeterServer};
use greeter::pb::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status, transport::Server};

struct MyGreeter;

impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        if name.is_empty() {
            return Err(Status::invalid_argument("name is required"));
        }
        Ok(Response::new(HelloReply {
            message: format!("Hello, {name}!"),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    println!("gRPC server listening on {addr}");
    Server::builder()
        .add_service(GreeterServer::new(MyGreeter))
        .serve(addr)
        .await?;
    Ok(())
}
