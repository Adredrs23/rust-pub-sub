use axum::extract::Query;
use axum::{
    Router,
    extract::Json,
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
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🔐 Auth service running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn register(
    Json(payload): Json<Registration>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> &'static str {
    let mut auth_list = state.authorized_emails.lock().unwrap();
    auth_list.insert(payload.email);
    "✅ Registered"
}

async fn is_authorized(
    Query(params): Query<Registration>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<bool> {
    let auth_list = state.authorized_emails.lock().unwrap();
    Json(auth_list.contains(&params.email))
}
