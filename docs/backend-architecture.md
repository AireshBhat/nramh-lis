# Laboratory Information System - Backend Architecture

## Overview

This document outlines the backend technical architecture for the Laboratory Information System (LIS) built using Tauri and Rust. The backend is designed with a layered, modular approach, following the system structure depicted in the `rust-core-architecture.mermaid` diagram. It emphasizes clear separation of concerns, robust data flow, and extensibility for laboratory and hospital integration.

## Technology Stack

- **Desktop Framework**: Tauri
- **Backend Language**: Rust
- **Database**: SQLite (via SQLx)
- **Async Runtime**: Tokio
- **Serialization**: Serde
- **Plugin System**: Tauri Plugins

## Layered Architecture

### 1. API Command Layer

Handles all commands invoked from the frontend or external systems via Tauri's command API.

```rust
// src-tauri/src/lib.rs
pub mod handlers;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Register commands
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            handlers::ip_handler::get_local_ip,
            // ...other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Service Orchestration Layer

Coordinates business logic for device management, data queries, configuration, system status, and uploads.

```rust
// src/services/device_orchestration.rs
pub struct DeviceOrchestrationService;

impl DeviceOrchestrationService {
    pub async fn manage_device(&self, device_id: &str) {
        // Orchestrate device actions
    }
}
```

### 3. Protocol Processing Layer

Handles ASTM and HL7 protocol parsing and validation.

```rust
// src/protocol/astm.rs
pub struct AstmProtocolHandler;

impl AstmProtocolHandler {
    pub fn parse_message(&self, raw: &str) -> Result<ParsedMessage, ProtocolError> {
        // Dummy parse logic
        Ok(ParsedMessage {})
    }
}
```

### 4. Connection Management Layer

Manages TCP/IP and Serial connections to laboratory devices.

```rust
// src/connection/tcp.rs
pub struct TcpConnectionManager;

impl TcpConnectionManager {
    pub async fn listen(&self, port: u16) {
        // Listen for device messages
    }
}
```

### 5. Message Processing Pipeline

Processes, validates, and transforms incoming messages.

```rust
// src/message/processing.rs
pub struct MessageParser;
pub struct MessageValidator;
pub struct MessageTransformer;

impl MessageParser {
    pub fn parse(&self, data: &[u8]) -> Result<ParsedData, ParseError> {
        // Dummy parse
        Ok(ParsedData {})
    }
}
```

### 6. Data Processing Layer

Resolves and processes patient, sample, and result data.

```rust
// src/data/processor.rs
pub struct DataProcessor;

impl DataProcessor {
    pub async fn process(&self, data: ParsedData) -> Result<(), DataError> {
        // Dummy processing
        Ok(())
    }
}
```

### 7. Repository Abstraction Layer

Implements the repository pattern for all core entities.

```rust
// src/storage/traits.rs
#[async_trait]
pub trait Repository<T> {
    async fn create(&self, entity: &T) -> Result<(), RepoError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, RepoError>;
    // ...
}
```

### 8. Data Persistence Layer

Persists data to SQLite using SQLx.

```rust
// src/storage/sqlite.rs
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
}
```

### 9. Event System

Publishes device, data, error, and system events.

```rust
// src/event/mod.rs
pub enum EventType {
    Device,
    Data,
    Error,
    System,
}

pub struct EventSystem;

impl EventSystem {
    pub fn publish(&self, event: EventType, payload: &str) {
        // Dummy publish
    }
}
```

### 10. Cross-Cutting Concerns

#### Logging
```rust
// src/logging.rs
pub struct LoggingService;

impl LoggingService {
    pub fn log(&self, message: &str) {
        // Dummy log
    }
}
```

#### Error Handling
```rust
// src/model/error.rs
#[derive(Debug)]
pub enum ModelError {
    NotFound,
    DatabaseError,
    // ...
}
```

#### Configuration
```rust
// src/config.rs
pub struct ConfigurationManager;

impl ConfigurationManager {
    pub fn get(&self, key: &str) -> Option<String> {
        // Dummy config
        None
    }
}
```

#### Caching
```rust
// src/cache.rs
pub struct CacheManager;

impl CacheManager {
    pub fn get(&self, key: &str) -> Option<String> {
        // Dummy cache
        None
    }
}
```

### 11. External Integration

Handles communication with Hospital Information Systems (HIS) and upload management.

```rust
// src/integration/his.rs
pub struct HisUploadService;

impl HisUploadService {
    pub async fn upload(&self, data: &str) -> Result<(), UploadError> {
        // Dummy upload
        Ok(())
    }
}
```

### 12. Health Monitoring

Monitors device, system, and database health.

```rust
// src/monitoring/health.rs
pub struct HealthCheckService;

impl HealthCheckService {
    pub async fn check(&self) -> HealthStatus {
        // Dummy health check
        HealthStatus::Ok
    }
}

pub enum HealthStatus {
    Ok,
    Warning,
    Critical,
}
```

## Database Schema (SQLite)

```sql
-- analyzers table
CREATE TABLE analyzers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    model TEXT NOT NULL,
    serial_number TEXT,
    manufacturer TEXT,
    connection_type TEXT NOT NULL,
    ip_address TEXT,
    port INTEGER,
    protocol TEXT NOT NULL,
    status TEXT NOT NULL,
    activate_on_start BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- patients table
CREATE TABLE patients (
    id TEXT PRIMARY KEY,
    last_name TEXT,
    first_name TEXT,
    middle_name TEXT,
    birth_date DATETIME,
    sex TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- test_results table
CREATE TABLE test_results (
    id TEXT PRIMARY KEY,
    test_id TEXT NOT NULL,
    sample_id TEXT NOT NULL,
    value TEXT NOT NULL,
    units TEXT,
    reference_range_lower REAL,
    reference_range_upper REAL,
    status TEXT NOT NULL,
    completed_date_time DATETIME,
    analyzer_id TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- upload_status table
CREATE TABLE upload_status (
    id TEXT PRIMARY KEY,
    result_id TEXT NOT NULL,
    external_system_id TEXT NOT NULL,
    status TEXT NOT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
```

## Patterns and Best Practices

- **Repository Pattern** for all data access
- **Service Layer** for orchestration and business logic
- **Protocol Handlers** for ASTM/HL7
- **Event System** for decoupled communication
- **Health Monitoring** for observability
- **Cross-cutting**: Logging, Error Handling, Config, Caching

---

This backend architecture ensures modularity, maintainability, and extensibility for laboratory data processing and integration with external systems. 