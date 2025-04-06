use async_nats::ConnectOptions;
use axum::extract::Path;
use axum::{Json, Router, extract::State, routing::get};
use futures::StreamExt;
use std::{collections::HashMap, net::SocketAddr};
use stock_ticker::types::{AggregatedState, AggregatedStats, StockPrice};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AggregatedState::default();

    // Spawn listener
    let nats_state = state.clone();
    task::spawn(async move {
        if let Err(err) = start_nats_listener(nats_state).await {
            eprintln!("NATS listener error: {:?}", err);
        }
    });

    // Set up API
    let app = Router::new()
        .route("/aggregate", get(get_stats))
        .route("/raw", get(get_raw))
        .route("/aggregate/{symbol}", get(get_stats_for_symbol))
        .route("/raw/{symbol}", get(get_raw_for_symbol))
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("📊 Aggregator service running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// GET /aggregate → returns only the stats
async fn get_stats(State(state): State<AggregatedState>) -> Json<HashMap<String, AggregatedStats>> {
    let data = state.stats_data.lock().unwrap();
    Json(data.clone())
}

// GET /raw → returns the raw stock price list
async fn get_raw(State(state): State<AggregatedState>) -> Json<HashMap<String, Vec<StockPrice>>> {
    let raw = state.raw_data.lock().unwrap();
    Json(raw.clone())
}

async fn start_nats_listener(state: AggregatedState) -> Result<(), Box<dyn std::error::Error>> {
    let client = ConnectOptions::new()
        .connect("nats://127.0.0.1:4222")
        .await?;

    println!("📡 Aggregator connected to NATS.");

    let mut subscriber = client.subscribe("stock_prices").await?;
    while let Some(message) = subscriber.next().await {
        let payload = String::from_utf8_lossy(&message.payload);
        if let Ok(stock) = serde_json::from_str::<StockPrice>(&payload) {
            {
                let mut raw_map = state.raw_data.lock().unwrap();
                raw_map
                    .entry(stock.symbol.clone())
                    .or_default()
                    .push(stock.clone());
            }

            {
                let mut stats_map = state.stats_data.lock().unwrap();
                let stats = stats_map
                    .entry(stock.symbol.clone())
                    .or_insert(AggregatedStats {
                        total: 0.0,
                        count: 0,
                        average: 0.0,
                        latest: 0.0,
                    });

                stats.total += stock.price;
                stats.count += 1;
                stats.latest = stock.price;
                stats.average = stats.total / stats.count as f64;
            }
        }
    }

    Ok(())
}

// GET /aggregate/:symbol → return stats for 1 symbol
async fn get_stats_for_symbol(
    State(state): State<AggregatedState>,
    Path(symbol): Path<String>,
) -> Json<Option<AggregatedStats>> {
    let data = state.stats_data.lock().unwrap();
    Json(data.get(&symbol).cloned())
}

// GET /raw/:symbol → return raw stock data for 1 symbol
async fn get_raw_for_symbol(
    State(state): State<AggregatedState>,
    Path(symbol): Path<String>,
) -> Json<Option<Vec<StockPrice>>> {
    let data = state.raw_data.lock().unwrap();
    Json(data.get(&symbol).cloned())
}
