# Building a Real-Time Stock Ticker System with Rust and NATS

In this post, we'll explore a real-time stock ticker system built with Rust, demonstrating the power of asynchronous programming and message-based architecture. The system consists of two main components: a publisher that generates simulated stock prices and a consumer that processes these updates.

## System Architecture

The system uses a publish-subscribe pattern with NATS (Neural Autonomic Transport System) as the message broker. This architecture allows for:

- Real-time data distribution
- Loose coupling between components
- Scalable message handling
- Reliable message delivery

## Components

### Publisher (`src/publisher.rs`)

The publisher is responsible for:

1. Generating simulated stock prices for a predefined set of symbols (AAPL, GOOGL, AMZN, MSFT, TSLA)
2. Publishing these prices to a NATS topic called "stock_prices"
3. Running in an infinite loop with a 1-second delay between updates

Key features:

- Asynchronous operation using Tokio runtime
- Random price generation between $100 and $500
- JSON serialization of stock price data
- Timestamp inclusion for each price update

### Consumer (`src/consumer.rs`)

The consumer:

1. Subscribes to the "stock_prices" topic
2. Receives and deserializes stock price updates
3. Prints the received data to the console

## Technical Implementation

### Stock Price Structure

```rust
#[derive(Serialize, Deserialize, Debug)]
struct StockPrice {
    symbol: String,
    price: f64,
    timestamp: String,
}
```

### Key Technologies Used

- **Tokio**: Asynchronous runtime for Rust
- **NATS**: High-performance messaging system
- **Serde**: Serialization/deserialization framework
- **Chrono**: Date and time library
- **Rand**: Random number generation

## Running the System

1. Start the NATS server (required)
2. Run the publisher:
   ```bash
   cargo run --bin publisher
   ```
3. Run the consumer:
   ```bash
   cargo run --bin consumer
   ```

## Future Enhancements

Potential improvements could include:

- Adding a database to store historical prices
- Implementing price alerts
- Adding a web interface for real-time visualization
- Supporting more stock symbols
- Adding authentication and authorization
- Implementing error recovery and reconnection logic

## Conclusion

This system demonstrates how to build a real-time data processing pipeline using Rust and NATS. The combination of Rust's performance and safety features with NATS's reliable messaging makes it suitable for production-grade financial applications.

The code is structured to be maintainable and extensible, making it a good foundation for building more complex financial data processing systems.
