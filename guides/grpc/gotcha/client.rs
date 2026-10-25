use greeter::pb::HelloRequest;
use greeter::pb::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let ana = HelloRequest { name: "Ana".into() };
    let reply = client.say_hello(ana).await?;
    println!("server said: {}", reply.into_inner().message);

    let nobody = HelloRequest::default(); // name is empty
    match client.say_hello(nobody).await {
        Ok(_) => println!("unexpected success"),
        Err(status) => println!("error {:?}: {}", status.code(), status.message()),
    }
    Ok(())
}
