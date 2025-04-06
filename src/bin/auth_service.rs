use axum::extract::Query;
use axum::{
    Router,
    extract::Json,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Registration {
    email: String,
}

#[derive(Clone)]
struct AppState {
    authorized_emails: Arc<Mutex<HashSet<String>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        authorized_emails: Arc::new(Mutex::new(HashSet::new())),
    };

    let app = Router::new()
        .route("/register", post(register))
        .route("/is-authorized", get(is_authorized))
        .route("/list-emails", get(list_emails))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🔐 Auth service running on http://{}", addr);

    // Updated server binding code for axum 0.8.1
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
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

async fn list_emails(State(state): State<AppState>) -> Json<Vec<String>> {
    let auth_list = state.authorized_emails.lock().unwrap();
    Json(auth_list.iter().cloned().collect())
}
