# Authentication Service Documentation

## Overview

The Authentication Service is a simple HTTP server that manages user authorization through email addresses. It provides endpoints for registering new users, checking their authorization status, and listing all authorized emails. This service is designed to be lightweight, fast, and thread-safe, making it suitable for integration with other components of the stock ticker system.

## Project Structure

The project follows a standard Rust project structure with some specific organization:

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

### Module Organization

- **Library Modules**: Shared code is placed in the `src` directory and exposed through `lib.rs`
- **Binary Modules**: Each binary has its own entry point in the `src/bin` directory
- **Import Pattern**: Binaries import from the library using `use stock_ticker::types::StockPrice;`

## Technical Stack

- **Framework**: Axum (Rust web framework)
- **Runtime**: Tokio for asynchronous operations
- **State Management**: Thread-safe shared state using `Arc<Mutex<>>`
- **Serialization**: Serde for JSON handling
- **Networking**: Standard library's SocketAddr and Tokio's TcpListener

## Architecture

The service follows a simple architecture:

1. **In-Memory Storage**: Uses a thread-safe HashSet to store authorized email addresses
2. **HTTP API**: Exposes three endpoints for registration, authorization checks, and listing emails
3. **State Management**: Shares state across all requests using Axum's state management

## Data Structures

### Registration

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Registration {
    email: String,
}
```

This struct represents a user registration request or query:

- `email`: The email address to register or check
- Implements `Debug`, `Deserialize`, `Serialize`, and `Clone` traits for logging, JSON handling, and state management

### AppState

```rust
#[derive(Clone)]
struct AppState {
    authorized_emails: Arc<Mutex<HashSet<String>>>,
}
```

This struct holds the application-wide state:

- `authorized_emails`: A thread-safe HashSet of authorized email addresses
- Uses `Arc<Mutex<>>` for safe concurrent access
- Implements `Clone` to allow sharing across request handlers

## API Endpoints

### 1. Register User

- **Endpoint**: `/register`
- **Method**: POST
- **Request Body**:
  ```json
  {
  	"email": "user@example.com"
  }
  ```
- **Response**: `"✅ Registered"`
- **Description**: Adds a new email to the authorized list
- **Handler Function**:
  ```rust
  async fn register(
      State(state): State<AppState>,
      Json(payload): Json<Registration>,
  ) -> &'static str {
      let mut auth_list = state.authorized_emails.lock().unwrap();
      auth_list.insert(payload.email);
      "✅ Registered"
  }
  ```

### 2. Check Authorization

- **Endpoint**: `/is-authorized`
- **Method**: GET
- **Query Parameters**: `email=user@example.com`
- **Response**: `true` or `false` (as JSON)
- **Description**: Checks if an email is in the authorized list
- **Handler Function**:
  ```rust
  async fn is_authorized(
      State(state): State<AppState>,
      Query(params): Query<Registration>,
  ) -> Json<bool> {
      let auth_list = state.authorized_emails.lock().unwrap();
      Json(auth_list.contains(&params.email))
  }
  ```

### 3. List Authorized Emails

- **Endpoint**: `/list-emails`
- **Method**: GET
- **Response**: Array of email strings (as JSON)
- **Description**: Returns a list of all authorized email addresses
- **Handler Function**:
  ```rust
  async fn list_emails(State(state): State<AppState>) -> Json<Vec<String>> {
      let auth_list = state.authorized_emails.lock().unwrap();
      Json(auth_list.iter().cloned().collect())
  }
  ```

## Server Configuration

- **Host**: 127.0.0.1
- **Port**: 3001
- **Base URL**: `http://127.0.0.1:3001`

## Request/Response Flow

### Registration Flow

1. Client sends a POST request to `/register` with a JSON body containing an email
2. Server deserializes the JSON into a `Registration` struct
3. Server acquires a lock on the authorized emails HashSet
4. Server adds the email to the HashSet
5. Server releases the lock
6. Server returns a success message

### Authorization Check Flow

1. Client sends a GET request to `/is-authorized?email=user@example.com`
2. Server deserializes the query parameters into a `Registration` struct
3. Server acquires a lock on the authorized emails HashSet
4. Server checks if the email exists in the HashSet
5. Server releases the lock
6. Server returns a JSON response with the result (true/false)

### List Emails Flow

1. Client sends a GET request to `/list-emails`
2. Server acquires a lock on the authorized emails HashSet
3. Server clones all emails from the HashSet into a new vector
4. Server releases the lock
5. Server returns a JSON response with the vector of emails

## Usage Examples

### Registering a User

```bash
curl -X POST http://127.0.0.1:3001/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'
```

Expected response:

```
✅ Registered
```

### Checking Authorization

```bash
curl "http://127.0.0.1:3001/is-authorized?email=user@example.com"
```

Expected response:

```json
true
```

### Listing Authorized Emails

```bash
curl "http://127.0.0.1:3001/list-emails"
```

Expected response:

```json
["user@example.com", "another@example.com"]
```

## Thread Safety

The service uses several mechanisms to ensure thread safety:

1. **Arc (Atomic Reference Counting)**: Allows multiple owners of the same data
2. **Mutex (Mutual Exclusion)**: Ensures only one thread can access the data at a time
3. **Clone for State**: Allows Axum to share state across request handlers

This combination ensures that the HashSet of authorized emails can be safely accessed and modified by multiple concurrent requests.

## Error Handling

The service uses Rust's Result type for error handling:

1. **Lock Errors**: Uses `unwrap()` on mutex locks (could be improved with proper error handling)
2. **Deserialization Errors**: Handled by Axum's extractors
3. **Server Errors**: Uses `unwrap()` on server startup (could be improved with proper error handling)

## Dependencies

```toml
[dependencies]
axum = { version = "0.8.1", features = ["macros"] }
serde = { version = "1.0.219", features = ["derive"] }
tokio = { version = "1.44.0", features = ["full"] }
```

## Running the Service

```bash
# Start the auth service
cargo run --bin auth_service
```

## Integration with Stock Ticker System

The auth service can be integrated with the stock ticker system to:

1. **Restrict Access**: Only allow authorized users to access stock price data
2. **User Management**: Register and manage users of the system
3. **Audit Trail**: Track which users are accessing the system

## Future Improvements

1. **Authentication**

   - Add password-based authentication
   - Implement JWT tokens
   - Add session management

2. **Storage**

   - Implement persistent storage (database)
   - Add data backup mechanisms

3. **Security**

   - Add HTTPS support
   - Implement rate limiting
   - Add input sanitization
   - Add logging and monitoring

4. **Features**
   - User roles and permissions
   - Email verification
   - Password reset functionality
   - Account management endpoints
