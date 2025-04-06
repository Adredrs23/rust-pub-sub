# Stock Price Consumer Documentation

## Overview

The Stock Price Consumer is a component that subscribes to real-time stock price updates from a NATS messaging system. It receives, deserializes, and processes stock price messages published by the Stock Price Publisher.

## Technical Stack

- **Messaging**: NATS (Neural Autonomic Transport System)
- **Runtime**: Tokio for asynchronous operations
- **Serialization**: Serde for JSON handling
- **Stream Processing**: Futures for async stream handling

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

- Shared with publisher component
- Deserializes incoming JSON messages
- Contains stock symbol, price, and timestamp

### Key Functions

#### Main Consumer Loop

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
```

- Connects to NATS server
- Subscribes to stock price topic
- Processes messages indefinitely
- Handles deserialization errors gracefully

## Configuration

- **NATS Server**: `nats://127.0.0.1:4222`
- **Topic**: `stock_prices`
- **Message Format**: JSON

## Message Processing

1. **Receiving**

   - Asynchronous message reception
   - UTF-8 payload conversion
   - Error handling for malformed messages

2. **Deserialization**

   - JSON to StockPrice struct conversion
   - Error handling for invalid JSON
   - Type validation

3. **Output**
   - Console logging of received prices
   - Error reporting for failed messages

## Error Handling

- NATS connection errors
- JSON deserialization errors
- Message processing errors
- Graceful error reporting with emoji indicators

## Dependencies

```toml
[dependencies]
async-nats = "0.39.0"
futures = "0.3.30"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
tokio = { version = "1.44.0", features = ["full"] }
```

## Usage

```bash
# Start the consumer
cargo run --bin consumer
```

## Output Example

```
Connected to NATS at nats://127.0.0.1:4222
Subscribed to 'stock_prices'...
📥 Received: StockPrice { symbol: "AAPL", price: 150.25, timestamp: "2024-04-06T14:30:00Z" }
📥 Received: StockPrice { symbol: "GOOGL", price: 2750.75, timestamp: "2024-04-06T14:30:00Z" }
❌ Failed to parse message: expected '}' at line 1, column 10
```

## Future Improvements

1. **Data Processing**

   - Price aggregation
   - Statistical analysis
   - Historical data storage

2. **Configuration**

   - Multiple topic subscriptions
   - Filtering options
   - Configurable processing rules

3. **Monitoring**

   - Message rate tracking
   - Latency monitoring
   - Error rate tracking

4. **Features**
   - Price alerts
   - Data visualization
   - Multiple output formats
   - Database integration
