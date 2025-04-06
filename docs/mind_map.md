# 📚 Rust Learning Summary: Stock Ticker System with Auth Service

## ✅ `#[derive(...)]`

- The `derive` attribute automatically implements common traits (e.g. `Debug`, `Serialize`, `Deserialize`, `Clone`) for structs and enums.
- This is essential for:
  - Debugging with `println!` or `dbg!`
  - Serializing/deserializing JSON with `serde`
  - Cloning state for sharing across handlers

### Example:

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Registration {
    email: String,
}
```

### Expanded Explanation

- `Debug`: Allows using `{:?}` format to print a struct's contents for debugging.

```rust
println!("{:?}", some_struct);
```

- `Serialize` and `Deserialize`: Required to convert struct to/from JSON.

```rust
let json = serde_json::to_string(&some_struct).unwrap();
let parsed: MyStruct = serde_json::from_str(&json).unwrap();
```

- `Clone`: Enables copying values instead of moving them, important for state sharing in handlers.

```rust
let a = some_struct.clone();
```

---

## ✅ Handlers with and without `AppState`

### With `AppState`:

- The shared application state is passed into handler functions using `State<T>`.
- This allows safe access to shared values (like authorized users).
- `AppState` typically holds data structures wrapped in `Arc<Mutex<_>>` for safe shared access.
- When using `AppState`, **it should always be the first parameter** in the handler function.

```rust
#[derive(Clone)]
struct AppState {
    authorized_emails: Arc<Mutex<HashSet<String>>>,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Registration>,
) -> &'static str {
    let mut auth_list = state.authorized_emails.lock().unwrap();
    auth_list.insert(payload.email);
    "✅ Registered"
}

async fn is_authorized(
    State(state): State<AppState>,
    Query(params): Query<Registration>,
) -> Json<bool> {
    let auth_list = state.authorized_emails.lock().unwrap();
    Json(auth_list.contains(&params.email))
}
```

### Without `AppState`:

- Routes can still be defined and executed, but they won’t share any runtime state.
- Handlers just take route-specific data like `Json` or `Query`.

```rust
async fn greet() -> &'static str {
    "Hello, world!"
}
```

---

## ✅ `Arc<Mutex<T>>` in AppState

### `Mutex<T>`

- Allows **interior mutability** — safe mutation of data within a `Mutex`.
- Only one thread/task can access it at a time.

### `Arc<T>`

- Reference-counted pointer to allow **shared ownership** of the state.
- Safe to clone and pass across async tasks or threads.

### Why Combine Them?

- `Arc<Mutex<T>>` allows **shared, mutable, and thread-safe** state for async apps like Axum.

### Example:

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

#[derive(Clone)]
struct AppState {
    authorized_emails: Arc<Mutex<HashSet<String>>>,
}
```

---

## ✅ Return Type: `&'static str`

- A handler returning `&'static str` indicates the response is a string **with a static lifetime**.
- It's often used for simple responses like:

```rust
async fn hello() -> &'static str {
    "Hello, world!"
}
```

- Safe to return hardcoded strings or statically stored string slices.

---

Let me know when you're ready to add more summary points! 🚀
