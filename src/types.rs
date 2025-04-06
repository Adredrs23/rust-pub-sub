use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AggregatedStats {
    pub total: f64,
    pub count: usize,
    pub average: f64,
    pub latest: f64,
}

#[derive(Clone, Default)]
pub struct AggregatedState {
    pub raw_data: Arc<Mutex<HashMap<String, Vec<StockPrice>>>>,
    pub stats_data: Arc<Mutex<HashMap<String, AggregatedStats>>>,
}
