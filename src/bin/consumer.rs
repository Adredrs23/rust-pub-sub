use async_nats::ConnectOptions;
use futures::StreamExt;
use reqwest;
use serde_json;
use std::env;

use stock_ticker::types::StockPrice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: consumer <email>");
        return Ok(());
    }

    let email = &args[1];
    let auth_url = format!("http://localhost:3001/is-authorized?email={}", email);

    let res = reqwest::get(&auth_url).await?.json::<bool>().await?;

    if !res {
        println!("❌ Access denied for {}", email);
        return Ok(());
    }

    println!("✅ Access granted. Connecting to NATS...");

    // Connect to the NATS server asynchronously
    let client = ConnectOptions::new()
        .connect("nats://127.0.0.1:4222")
        .await?;

    println!("Connected to NATS at nats://127.0.0.1:4222");

    // Subscribe to "stock_prices" topic
    let mut subscriber = client.subscribe("stock_prices").await?;
    println!("Subscribed to 'stock_prices'...");

    // Loop over incoming messages
    while let Some(message) = subscriber.next().await {
        let payload = String::from_utf8_lossy(&message.payload);

        match serde_json::from_str::<StockPrice>(&payload) {
            Ok(stock_price) => {
                println!("📥 Received: {:?}", stock_price);
            }
            Err(e) => {
                eprintln!("❌ Failed to parse message: {}", e);
            }
        }
    }

    Ok(())
}
