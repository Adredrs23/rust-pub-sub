# Stock Price Publisher Documentation

## Overview

The Stock Price Publisher is a component that simulates a real-time stock market data feed. It generates random stock prices for a predefined set of symbols and publishes them to a NATS messaging system.

## Technical Stack

- **Messaging**: NATS (Neural Autonomic Transport System)
- **Runtime**: Tokio for asynchronous operations
- **Serialization**: Serde for JSON handling
- **Random Generation**: Rand crate for price simulation
- **Time Handling**: Chrono for timestamp generation

## Components

### Data Structure

The `StockPrice` struct is defined in a shared module (`src/types.rs`):

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub timestamp: String,
}
```

- `symbol`: Stock ticker symbol (e.g., AAPL, GOOGL)
- `price`: Current stock price
- `timestamp`: ISO 8601 formatted timestamp

### Key Functions

#### Random Price Generation

```rust
async fn generate_random_price() -> f64 {
    let mut rng = rng();
    let price_range = Uniform::new(100.0, 500.0).expect("Failed to create uniform distribution");
    price_range.sample(&mut rng)
}
```

- Generates random prices between $100 and $500
- Uses the `rng()` function for random number generation
- Returns a floating-point price value

#### Main Publisher Loop

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

        // Sleep asynchronously for 2 seconds before generating new prices
        sleep(Duration::from_secs(2)).await;
    }
}
```

- Connects to NATS server
- Maintains list of stock symbols
- Publishes prices every 2 seconds
- Runs indefinitely

## Configuration

- **NATS Server**: `nats://127.0.0.1:4222`
- **Topic**: `stock_prices`
- **Update Interval**: 2 seconds
- **Supported Symbols**: AAPL, GOOGL, AMZN, MSFT, TSLA

## Message Format

```json
{
	"symbol": "AAPL",
	"price": 150.25,
	"timestamp": "2024-04-06T14:30:00Z"
}
```

## Error Handling

- NATS connection errors
- JSON serialization errors
- Random number generation errors

## Dependencies

```toml
[dependencies]
async-nats = "0.39.0"
chrono = "0.4.40"
rand = "0.9.0"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
tokio = { version = "1.44.0", features = ["full"] }
```

## Usage

```bash
# Start the publisher
cargo run --bin publisher
```

## Output Example

```
📤 Published: StockPrice { symbol: "AAPL", price: 150.25, timestamp: "2024-04-06T14:30:00Z" }
📤 Published: StockPrice { symbol: "GOOGL", price: 2750.75, timestamp: "2024-04-06T14:30:00Z" }
```

## Future Improvements

1. **Price Simulation**

   - More realistic price movements
   - Historical data integration
   - Market hours simulation

2. **Configuration**

   - Configurable symbols
   - Adjustable update intervals
   - Environment-based settings

3. **Monitoring**

   - Performance metrics
   - Error reporting
   - Health checks

4. **Features**
   - Multiple price ranges per symbol
   - Volume data
   - Market events
   - Price alerts
