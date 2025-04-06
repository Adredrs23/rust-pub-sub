# Building a Real-Time Stock Ticker System with Rust and NATS

In this post, we'll explore a real-time stock ticker system built with Rust, demonstrating the power of asynchronous programming and message-based architecture. The system consists of three main components: a publisher that generates simulated stock prices, a consumer that processes these updates, and an authentication service that manages user access.

## System Architecture

The system uses a publish-subscribe pattern with NATS (Neural Autonomic Transport System) as the message broker. This architecture allows for:

- Real-time data distribution
- Loose coupling between components
- Scalable message handling
- Reliable message delivery

## Project Structure

The project follows a standard Rust project structure with a library crate and multiple binary targets:

### Library Crate

The project includes a library crate defined in `src/lib.rs` that contains shared code used by multiple binaries:

```rust
// src/lib.rs
pub mod types;
```

This library crate exposes the `types` module, which contains shared data structures like `StockPrice` used across different components of the system.

### Binary Targets

The project defines multiple binary targets in `Cargo.toml`:

```toml
[[bin]]
name = "publisher"
path = "src/bin/publisher.rs"

[[bin]]
name = "consumer"
path = "src/bin/consumer.rs"

[[bin]]
name = "auth_service"
path = "src/bin/auth_service.rs"

[lib]
name = "stock_ticker"
path = "src/lib.rs"
```

This configuration allows each binary to be run independently while sharing code from the library crate.

## Components

### Publisher (`src/bin/publisher.rs`)

The publisher is responsible for:

1. Generating simulated stock prices for a predefined set of symbols (AAPL, GOOGL, AMZN, MSFT, TSLA)
2. Publishing these prices to a NATS topic called "stock_prices"
3. Running in an infinite loop with a 1-second delay between updates

Key features:

- Asynchronous operation using Tokio runtime
- Random price generation between $100 and $500
- JSON serialization of stock price data
- Timestamp inclusion for each price update

### Consumer (`src/bin/consumer.rs`)

The consumer:

1. Subscribes to the "stock_prices" topic
2. Receives and deserializes stock price updates
3. Prints the received data to the console
4. Requires authentication via the auth service

### Authentication Service (`src/bin/auth_service.rs`)

The authentication service:

1. Provides a simple HTTP API for user registration and authorization
2. Maintains a list of authorized email addresses
3. Allows the consumer to verify if a user is authorized to access stock data

## Technical Implementation

### Stock Price Structure

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub timestamp: String,
}
```

### Key Technologies Used

- **Tokio**: Asynchronous runtime for Rust
- **NATS**: High-performance messaging system
- **Serde**: Serialization/deserialization framework
- **Chrono**: Date and time library
- **Rand**: Random number generation
- **Axum**: Web framework for the authentication service
- **Reqwest**: HTTP client for the consumer

## Running the System

1. Start the NATS server (required)
2. Start the authentication service:
   ```bash
   cargo run --bin auth_service
   ```
3. Register a user with the auth service:
   ```bash
   curl -X POST http://127.0.0.1:3001/register \
     -H "Content-Type: application/json" \
     -d '{"email": "user@example.com"}'
   ```
4. Run the publisher:
   ```bash
   cargo run --bin publisher
   ```
5. Run the consumer with an authorized email:
   ```bash
   cargo run --bin consumer user@example.com
   ```

## Future Enhancements

Potential improvements could include:

- Adding a database to store historical prices
- Implementing price alerts
- Adding a web interface for real-time visualization
- Supporting more stock symbols
- Enhancing authentication with passwords and JWT tokens
- Implementing error recovery and reconnection logic
- Adding persistent storage for authorized users

## Conclusion

This system demonstrates how to build a real-time data processing pipeline using Rust and NATS. The combination of Rust's performance and safety features with NATS's reliable messaging makes it suitable for production-grade financial applications.

The code is structured to be maintainable and extensible, making it a good foundation for building more complex financial data processing systems.
