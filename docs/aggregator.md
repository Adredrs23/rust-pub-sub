# Aggregator Service Documentation

## Overview

The Aggregator Service is a component of the Stock Ticker System that collects, processes, and provides access to aggregated stock price data. It subscribes to the same NATS topic as the consumer but instead of just displaying the data, it maintains in-memory statistics and historical data for each stock symbol. The service exposes a REST API that allows clients to retrieve both raw and aggregated data.

## Architecture

The Aggregator Service follows a dual-purpose architecture:

1. **NATS Subscriber**: Continuously listens for stock price updates on the "stock_prices" topic
2. **HTTP API Server**: Provides endpoints for retrieving both raw and aggregated data

This architecture allows the service to:

- Collect real-time data without blocking the publisher
- Process and aggregate data as it arrives
- Serve data to multiple clients through a REST API

## Data Structures

### AggregatedStats

```rust
#[derive(Debug, Clone, Serialize)]
pub struct AggregatedStats {
    pub total: f64,
    pub count: usize,
    pub average: f64,
    pub latest: f64,
}
```

This struct holds statistical information for a stock symbol:

- `total`: Sum of all prices received
- `count`: Number of price updates received
- `average`: Average price (total / count)
- `latest`: Most recent price received

### AggregatedState

```rust
#[derive(Clone, Default)]
pub struct AggregatedState {
    pub raw_data: Arc<Mutex<HashMap<String, Vec<StockPrice>>>>,
    pub stats_data: Arc<Mutex<HashMap<String, AggregatedStats>>>,
}
```

This struct maintains the application state:

- `raw_data`: Thread-safe HashMap storing raw stock price history by symbol
- `stats_data`: Thread-safe HashMap storing aggregated statistics by symbol
- Implements `Clone` and `Default` traits for easy initialization and sharing

## API Endpoints

### 1. Get Aggregated Statistics

- **Endpoint**: `/aggregate`
- **Method**: GET
- **Response**: JSON object mapping symbols to their aggregated statistics
- **Description**: Returns aggregated statistics for all stock symbols
- **Handler Function**:
  ```rust
  async fn get_stats(State(state): State<AggregatedState>) -> Json<HashMap<String, AggregatedStats>> {
      let data = state.stats_data.lock().unwrap();
      Json(data.clone())
  }
  ```

### 2. Get Raw Data

- **Endpoint**: `/raw`
- **Method**: GET
- **Response**: JSON object mapping symbols to arrays of stock prices
- **Description**: Returns raw stock price history for all symbols
- **Handler Function**:
  ```rust
  async fn get_raw(State(state): State<AggregatedState>) -> Json<HashMap<String, Vec<StockPrice>>> {
      let raw = state.raw_data.lock().unwrap();
      Json(raw.clone())
  }
  ```

### 3. Get Aggregated Statistics for Symbol

- **Endpoint**: `/aggregate/{symbol}`
- **Method**: GET
- **Response**: JSON object containing aggregated statistics for the specified symbol
- **Description**: Returns aggregated statistics for a specific stock symbol
- **Handler Function**:
  ```rust
  async fn get_stats_for_symbol(
      State(state): State<AggregatedState>,
      Path(symbol): Path<String>,
  ) -> Json<Option<AggregatedStats>> {
      let data = state.stats_data.lock().unwrap();
      Json(data.get(&symbol).cloned())
  }
  ```

### 4. Get Raw Data for Symbol

- **Endpoint**: `/raw/{symbol}`
- **Method**: GET
- **Response**: JSON array of stock prices for the specified symbol
- **Description**: Returns raw stock price history for a specific symbol
- **Handler Function**:
  ```rust
  async fn get_raw_for_symbol(
      State(state): State<AggregatedState>,
      Path(symbol): Path<String>,
  ) -> Json<Option<Vec<StockPrice>>> {
      let data = state.raw_data.lock().unwrap();
      Json(data.get(&symbol).cloned())
  }
  ```

## Server Configuration

- **Host**: 127.0.0.1
- **Port**: 3002
- **Base URL**: `http://127.0.0.1:3002`

## Data Processing Flow

### NATS Subscription Flow

1. Service connects to the NATS server
2. Subscribes to the "stock_prices" topic
3. For each message received:
   - Deserializes the message into a `StockPrice` struct
   - Updates the raw data by adding the price to the symbol's history
   - Updates the aggregated statistics for the symbol

### Statistics Calculation

For each stock price update, the service:

1. Adds the price to the symbol's raw data history
2. Updates the symbol's statistics:
   - Increments the count
   - Adds the price to the total
   - Updates the latest price
   - Recalculates the average (total / count)

## Usage Examples

### Getting Aggregated Statistics

```bash
curl "http://127.0.0.1:3002/aggregate"
```

Expected response:

```json
{
	"AAPL": {
		"total": 1250.75,
		"count": 5,
		"average": 250.15,
		"latest": 252.3
	},
	"GOOGL": {
		"total": 3750.25,
		"count": 5,
		"average": 750.05,
		"latest": 755.8
	}
}
```

### Getting Raw Data

```bash
curl "http://127.0.0.1:3002/raw"
```

Expected response:

```json
{
	"AAPL": [
		{
			"symbol": "AAPL",
			"price": 248.5,
			"timestamp": "2023-06-15T14:30:00Z"
		},
		{
			"symbol": "AAPL",
			"price": 249.75,
			"timestamp": "2023-06-15T14:31:00Z"
		}
	]
}
```

### Getting Statistics for a Specific Symbol

```bash
curl "http://127.0.0.1:3002/aggregate/AAPL"
```

Expected response:

```json
{
	"total": 1250.75,
	"count": 5,
	"average": 250.15,
	"latest": 252.3
}
```

### Getting Raw Data for a Specific Symbol

```bash
curl "http://127.0.0.1:3002/raw/AAPL"
```

Expected response:

```json
[
	{
		"symbol": "AAPL",
		"price": 248.5,
		"timestamp": "2023-06-15T14:30:00Z"
	},
	{
		"symbol": "AAPL",
		"price": 249.75,
		"timestamp": "2023-06-15T14:31:00Z"
	}
]
```

## Thread Safety

The service uses several mechanisms to ensure thread safety:

1. **Arc (Atomic Reference Counting)**: Allows multiple owners of the same data
2. **Mutex (Mutual Exclusion)**: Ensures only one thread can access the data at a time
3. **Clone for State**: Allows Axum to share state across request handlers

This combination ensures that the HashMaps can be safely accessed and modified by multiple concurrent requests and the NATS subscriber.

## Error Handling

The service uses Rust's Result type for error handling:

1. **Lock Errors**: Uses `unwrap()` on mutex locks (could be improved with proper error handling)
2. **Deserialization Errors**: Handled by pattern matching on the result
3. **Server Errors**: Uses `?` operator for propagating errors

## Dependencies

```toml
[dependencies]
async-nats = "0.39.0"
axum = { version = "0.8.1", features = ["macros"] }
futures = "0.3.30"
tokio = { version = "1.44.0", features = ["full"] }
```

## Running the Service

```bash
# Start the aggregator service
cargo run --bin aggregator
```

## Integration with Stock Ticker System

The aggregator service can be integrated with the stock ticker system to:

1. **Data Analysis**: Provide statistical insights into stock price movements
2. **Historical Data**: Maintain a history of stock prices for analysis
3. **API Access**: Allow other services to retrieve stock data through a REST API

## Future Improvements

1. **Data Persistence**

   - Implement database storage for historical data
   - Add data retention policies

2. **Advanced Analytics**

   - Add more statistical measures (median, mode, standard deviation)
   - Implement trend analysis and forecasting

3. **Performance Optimization**

   - Implement data pruning for long-running services
   - Add caching for frequently accessed data

4. **API Enhancements**
   - Add filtering and pagination for large datasets
   - Implement WebSocket for real-time updates
   - Add authentication for API access
