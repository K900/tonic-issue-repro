use std::net::SocketAddr;
use tonic::transport::Endpoint;
use warp::Filter;

mod proto;
use proto::greeter_client::GreeterClient;
use proto::HelloRequest;

async fn say_hello(
    mut client: proto::greeter_client::GreeterClient<tonic::transport::Channel>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = client
        .say_hello(HelloRequest {
            name: "World".to_string(),
        })
        .await;
    Ok(format!("{:?}\n", response))
}

#[tokio::main]
async fn main() {
    let endpoint = Endpoint::from_static("grpc://localhost:50051");
    let channel = endpoint
        .connect_lazy()
        .expect("Failed to connect to upstream!");
    let client = GreeterClient::new(channel);

    let routes = warp::path("hello")
        .map(move || client.clone())
        .and_then(say_hello);

    let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    println!("GreeterProxy listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
