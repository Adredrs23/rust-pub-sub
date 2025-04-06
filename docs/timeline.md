# Stock Ticker System Development Timeline

This document outlines the key milestones and development steps that led to the current state of the Stock Ticker System.

## Phase 1: Initial Setup and Basic Structure

### Milestone 1: Project Initialization

- Created a new Rust project using `cargo new stock-ticker`
- Set up the basic project structure with `src/main.rs`
- Initialized Git repository for version control

### Milestone 2: Core Dependencies

- Added essential dependencies to `Cargo.toml`:
  - `tokio` for asynchronous runtime
  - `serde` and `serde_json` for serialization
  - `async-nats` for message broker integration
  - `chrono` for timestamp handling
  - `rand` for random price generation

## Phase 2: Publisher Implementation

### Milestone 3: Stock Price Data Structure

- Created the `StockPrice` struct in `src/types.rs`:
  ```rust
  #[derive(Serialize, Deserialize, Debug)]
  pub struct StockPrice {
      pub symbol: String,
      pub price: f64,
      pub timestamp: String,
  }
  ```
- Implemented serialization/deserialization for JSON format

### Milestone 4: Publisher Service

- Implemented the publisher binary in `src/bin/publisher.rs`
- Added functionality to:
  - Connect to NATS server
  - Generate random stock prices for predefined symbols
  - Publish prices to the "stock_prices" topic
  - Run in an infinite loop with a delay between updates

## Phase 3: Consumer Implementation

### Milestone 5: Consumer Service

- Implemented the consumer binary in `src/bin/consumer.rs`
- Added functionality to:
  - Connect to NATS server
  - Subscribe to the "stock_prices" topic
  - Deserialize and process incoming stock price updates
  - Display the received data in the console

## Phase 4: Authentication Service

### Milestone 6: Authentication Service Setup

- Added Axum web framework dependency to `Cargo.toml`
- Created the authentication service binary in `src/bin/auth_service.rs`
- Implemented basic HTTP server with Axum

### Milestone 7: Authentication Endpoints

- Implemented the `/register` endpoint for adding authorized emails
- Implemented the `/is-authorized` endpoint for checking authorization
- Added thread-safe state management using `Arc<Mutex<>>`

### Milestone 8: Consumer Authentication Integration

- Updated the consumer to require an email parameter
- Added authentication check before connecting to NATS
- Implemented error handling for unauthorized access

### Milestone 9: List Emails Endpoint

- Added the `/list-emails` endpoint to the authentication service
- Implemented functionality to return all authorized emails

## Phase 5: Project Structure Refinement

### Milestone 10: Library Crate Implementation

- Created `src/lib.rs` to serve as the library crate entry point
- Moved shared code (like `StockPrice` struct) to the library crate
- Updated `Cargo.toml` to include the library crate configuration:
  ```toml
  [lib]
  name = "stock_ticker"
  path = "src/lib.rs"
  ```

### Milestone 11: Binary Target Configuration

- Updated `Cargo.toml` to properly define binary targets:

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
  ```

### Milestone 12: Import Path Refinement

- Updated import paths in binaries to use the library crate:
  ```rust
  use stock_ticker::types::StockPrice;
  ```
- Removed redundant module declarations

## Phase 6: Documentation

### Milestone 13: README Creation

- Created comprehensive README.md with:
  - System architecture overview
  - Component descriptions
  - Running instructions
  - Future enhancement ideas

### Milestone 14: Authentication Service Documentation

- Created detailed documentation for the authentication service in `docs/auth_service.md`
- Included API endpoints, data structures, and usage examples

### Milestone 15: Timeline Documentation

- Created this timeline document to track the project's evolution

## Current State

The Stock Ticker System now consists of three main components:

1. **Publisher**: Generates and publishes simulated stock prices to NATS
2. **Consumer**: Subscribes to stock prices and displays them (with authentication)
3. **Authentication Service**: Manages user authorization through a REST API

The project follows a well-structured organization with:

- A library crate for shared code
- Multiple binary targets for different services
- Comprehensive documentation

## Next Steps

Future development will focus on:

1. **Enhanced Authentication**:

   - Password-based authentication
   - JWT token implementation
   - Session management

2. **Data Persistence**:

   - Database integration for storing historical prices
   - Persistent storage for authorized users

3. **User Interface**:

   - Web interface for real-time visualization
   - User management dashboard

4. **Security Enhancements**:

   - HTTPS support
   - Rate limiting
   - Input sanitization

5. **Error Handling**:
   - Improved error handling throughout the system
   - Logging and monitoring
