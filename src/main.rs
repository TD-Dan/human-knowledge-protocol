//! Human knowledge protocol client
mod network;

#[tokio::main]
async fn main() {
    
    println!("Human Knowledge Protocol\n");

    network::test_connection().await;
}
