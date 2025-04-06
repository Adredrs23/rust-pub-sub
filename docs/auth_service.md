# Authentication Service Documentation

## Overview

The Authentication Service is a simple HTTP server that manages user authorization through email addresses. It provides endpoints for registering new users and checking their authorization status.

## Technical Stack

- **Framework**: Axum (Rust web framework)
- **State Management**: Thread-safe shared state using `Arc<Mutex<>>`
- **Serialization**: Serde for JSON handling
- **Runtime**: Tokio for async operations

## Components

### Data Structures

#### Registration

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Registration {
    email: String,
}
```

- Represents a user registration request
- Contains email address for identification
- Implements Debug, Deserialize, Serialize, and Clone traits

#### AppState

```rust
#[derive(Clone)]
struct AppState {
    authorized_emails: Arc<Mutex<HashSet<String>>>,
}
```

- Holds application-wide state
- Uses `Arc<Mutex<>>` for thread-safe access
- Stores authorized emails in a HashSet

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

### 2. Check Authorization

- **Endpoint**: `/is-authorized`
- **Method**: GET
- **Query Parameters**: `email=user@example.com`
- **Response**: `true` or `false`
- **Description**: Checks if an email is in the authorized list

## Server Configuration

- **Host**: 127.0.0.1
- **Port**: 3001
- **Base URL**: `http://127.0.0.1:3001`

## Usage Examples

### Registering a User

```bash
curl -X POST http://127.0.0.1:3001/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'
```

### Checking Authorization

```bash
curl "http://127.0.0.1:3001/is-authorized?email=user@example.com"
```

## Security Considerations

1. **Thread Safety**

   - Uses `Arc<Mutex<>>` for safe concurrent access
   - Prevents race conditions in email list modifications

2. **Input Validation**

   - JSON deserialization through Serde
   - Query parameter validation

3. **Current Limitations**
   - In-memory storage (data lost on restart)
   - No password authentication
   - No HTTPS
   - No rate limiting

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

## Error Handling

The service currently uses basic error handling. Future versions should include:

- Detailed error messages
- HTTP status codes
- Error logging
- Retry mechanisms

## Dependencies

```toml
[dependencies]
axum = "0.8.1"
serde = { version = "1.0.219", features = ["derive"] }
tokio = { version = "1.44.0", features = ["full"] }
```
