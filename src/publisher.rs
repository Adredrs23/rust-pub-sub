use async_nats::connect;
use chrono::Utc;
use rand::distr::{Distribution, Uniform};
use rand::rng;
use serde_json;
use tokio::time::{Duration, sleep};

mod types;
use types::StockPrice;

async fn generate_random_price() -> f64 {
    let mut rng = rng();
    let price_range = Uniform::new(100.0, 500.0).expect("Failed to create uniform distribution");
    price_range.sample(&mut rng)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to NATS server asynchronously
    let client = connect("nats://127.0.0.1:4222").await?;

    let symbols = vec!["AAPL", "GOOGL", "AMZN", "MSFT", "TSLA"];

    loop {
        for symbol in &symbols {
            let stock_price = StockPrice {
                symbol: symbol.to_string(),
                price: generate_random_price().await,
                timestamp: Utc::now().to_rfc3339(),
            };

            // Serialize struct to JSON
            let message = serde_json::to_string(&stock_price)?;

            // Publish to NATS asynchronously
            client.publish("stock_prices", message.into()).await?;

            println!("📤 Published: {:?}", stock_price);
        }

        // Sleep asynchronously for 1 second before generating new prices
        sleep(Duration::from_secs(2)).await;
    }
}
