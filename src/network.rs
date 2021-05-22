//! Network connection to iota ledger
//! Manages all in / out traffic

use iota_client::Client;

pub async fn test_connection() {
    println!("testing connection to node..");
    
    let iota = Client::builder()
    .with_node("https://api.lb-0.testnet.chrysalis2.com")
    .unwrap()
    .finish()
    .await
    .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Node Info: {:?}", info);
}