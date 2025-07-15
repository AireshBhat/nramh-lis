# BF-6900 Hematology Analyzer Implementation Guide

## Project Context

### Overview
This project is a Laboratory Information System (LIS) middleware built with Tauri (Rust backend + Web frontend) that interfaces with laboratory analyzers. The system currently supports the Meril AutoQuant analyzer using the ASTM protocol and is being extended to support the BF-6900 Hematology analyzer using the HL7 protocol.

### Current Architecture

#### Technology Stack
- **Backend:** Rust with Tauri framework
- **Frontend:** Web technologies (React/TypeScript implied from tsconfig.json)
- **Database:** SQLite (see migrations.rs)
- **Configuration:** JSON-based storage with tauri-plugin-store
- **Communication:** TCP/IP sockets with async/await patterns using Tokio

#### Existing Implementation Analysis

The current Meril AutoQuant implementation provides an excellent architectural foundation:

**Service Layer (`src-tauri/src/services/autoquant_meril.rs`):**
- Event-driven architecture with `MerilEvent` enum for type-safe communication
- Async TCP listener using tokio for non-blocking I/O
- State machine for ASTM protocol handling (`ConnectionState` enum)
- Comprehensive error handling and logging
- Frontend communication via Tauri events

**Command Layer (`src-tauri/src/api/commands/meril_handler.rs`):**
- Tauri commands for frontend-backend communication
- Configuration validation and persistence
- Service lifecycle management (start/stop)
- Status monitoring and reporting

**App State Management (`src-tauri/src/app_state.rs`):**
- Centralized state management with Arc<RwLock<>> for thread safety
- Service initialization and lifecycle coordination
- Event forwarding to frontend
- Auto-start configuration support

### Key Design Patterns

#### 1. Event-Driven Architecture
```rust
pub enum MerilEvent {
    AnalyzerConnected { analyzer_id: String, remote_addr: String, timestamp: DateTime<Utc> },
    AstmMessageReceived { analyzer_id: String, message_type: String, raw_data: String, timestamp: DateTime<Utc> },
    LabResultProcessed { analyzer_id: String, patient_data: Option<PatientData>, test_results: Vec<TestResult>, timestamp: DateTime<Utc> },
    // ... more events
}
```

#### 2. Protocol State Machine
```rust
pub enum ConnectionState {
    WaitingForEnq,      // Waiting for ENQ (Enquiry)
    WaitingForFrame,    // Waiting for STX (Start of Text)
    ProcessingFrame,    // Processing data frame
    WaitingForChecksum, // Waiting for checksum validation
    WaitingForCR,       // Waiting for Carriage Return
    WaitingForLF,       // Waiting for Line Feed
    Complete,           // Transmission complete
}
```

#### 3. Async Service Pattern
```rust
pub struct AutoQuantMerilService<R: Runtime> {
    analyzer: Arc<RwLock<Analyzer>>,
    listener: Arc<Mutex<Option<TcpListener>>>,
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    event_sender: mpsc::Sender<MerilEvent>,
    is_running: Arc<RwLock<bool>>,
    store: Arc<tauri_plugin_store::Store<R>>,
}
```

### Protocol Comparison: ASTM vs HL7

#### ASTM Protocol (Current - Meril AutoQuant)
- **Frame Structure:** `<FrameNumber><STX><Data><ETX><Checksum><CR><LF>`
- **Message Format:** Record-based with specific field positions
- **Handshake:** ENQ/ACK protocol for transmission control
- **Records:** H (Header), P (Patient), O (Order), R (Result), L (Terminator)

#### HL7 Protocol (Target - BF-6900)
- **Frame Structure:** `<VT><Message><FS><CR>` (MLLP)
- **Message Format:** Pipe-delimited segments with structured fields
- **Handshake:** MLLP acknowledgment with ACK/NAK responses
- **Segments:** MSH (Header), PID (Patient), OBR (Order), OBX (Observation)

### Database Schema

The existing schema supports the analyzer abstraction:

```sql
CREATE TABLE analyzers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    model TEXT NOT NULL,
    serial_number TEXT,
    manufacturer TEXT,
    connection_type TEXT NOT NULL, -- 'TCP_IP' or 'Serial'
    ip_address TEXT,
    port INTEGER,
    com_port TEXT,
    baud_rate INTEGER,
    protocol TEXT NOT NULL,        -- 'ASTM' or 'HL7'
    status TEXT NOT NULL,          -- 'Active', 'Inactive', 'Error'
    activate_on_start BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

### Configuration Management

#### Current Configuration Pattern
```json
{
  "analyzer": {
    "id": "meril-autoquant-001",
    "name": "AutoQuant",
    "model": "200i",
    "manufacturer": "Meril Diagnostics PVT LTD",
    "connection_type": "TcpIp",
    "ip_address": "192.168.1.100",
    "port": 5600,
    "protocol": "Astm",
    "status": "Inactive",
    "activate_on_start": true
  }
}
```

#### Target BF-6900 Configuration
```json
{
  "analyzer": {
    "id": "bf6900-hematology-001",
    "name": "BF-6900 Hematology Analyzer",
    "model": "BF-6900",
    "manufacturer": "Mindray",
    "connection_type": "TcpIp",
    "ip_address": "192.168.1.101",
    "port": 9100,
    "protocol": "HL7_V24",
    "status": "Inactive",
    "activate_on_start": false
  },
  "hl7_settings": {
    "mllp_enabled": true,
    "timeout_ms": 10000,
    "retry_attempts": 3,
    "encoding": "UTF-8",
    "message_types": ["ORU^R01", "OUL^R21"]
  }
}
```

### Data Flow Architecture

#### Current ASTM Flow (Meril)
1. **Connection:** TCP listener accepts analyzer connection
2. **Handshake:** ASTM ENQ/ACK protocol establishment
3. **Data Reception:** Frame-by-frame ASTM message collection
4. **Parsing:** Extract patient and result data from ASTM records
5. **Processing:** Transform data into internal format
6. **Storage:** Persist to SQLite database
7. **Notification:** Emit events to frontend via Tauri

#### Target HL7 Flow (BF-6900)
1. **Connection:** TCP listener accepts analyzer connection
2. **Handshake:** MLLP protocol establishment
3. **Data Reception:** HL7 message collection with MLLP framing
4. **Parsing:** Extract patient and result data from HL7 segments
5. **Processing:** Transform hematology data into internal format
6. **Storage:** Persist to SQLite database
7. **Acknowledgment:** Send HL7 ACK/NAK response
8. **Notification:** Emit events to frontend via Tauri

### Error Handling Strategy

The existing implementation demonstrates robust error handling:

#### Connection Level
```rust
match timeout(Duration::from_secs(5), connection.stream.read(&mut buffer)).await {
    Ok(Ok(0)) => {
        // Connection closed
        log::info!("Connection closed by {}", connection.remote_addr);
        break;
    }
    Ok(Ok(n)) => {
        // Process data
        let data = &buffer[..n];
        if let Err(e) = Self::process_astm_data(connection, data, &event_sender).await {
            log::error!("Error processing ASTM data: {}", e);
            // Send error event to frontend
        }
    }
    Ok(Err(e)) => {
        log::error!("Error reading from connection: {}", e);
        break;
    }
    Err(_) => {
        // Timeout, continue
        continue;
    }
}
```

#### Protocol Level
- Checksum validation for data integrity
- State machine error recovery
- Graceful connection cleanup
- Frontend error notification

### Testing Strategy

#### Current Testing Approach
The existing code includes unit tests for:
- Configuration validation
- IP address and port validation
- Protocol-specific validation functions

#### Required Testing for BF-6900
- HL7 message parsing unit tests
- MLLP frame handling tests
- Connection state machine tests
- Integration tests with mock HL7 server
- End-to-end testing with real analyzer

### Frontend Integration

#### Event Communication Pattern
The backend communicates with the frontend via Tauri events:

```rust
// Backend event emission
let _ = app.emit(
    "meril:lab-results",
    serde_json::json!({
        "analyzer_id": analyzer_id,
        "patient_data": patient_data,
        "test_results": test_results,
        "timestamp": timestamp
    }),
);
```

#### Command Pattern
Frontend can control the backend via Tauri commands:

```rust
#[tauri::command]
pub async fn start_meril_service<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    // Service start logic
}
```

### Performance Considerations

#### Memory Management
- Arc<RwLock<>> for shared state across threads
- Connection pooling with HashMap storage
- Automatic cleanup of disconnected analyzers

#### Concurrency
- Async/await throughout for non-blocking I/O
- Separate tasks for each analyzer connection
- Event-driven architecture prevents blocking

#### Scalability
- Multiple analyzer support (Meril + BF-6900 + future analyzers)
- Configurable timeouts and retry logic
- Resource cleanup and connection limits

### Security Considerations

#### Network Security
- IP address validation for connections
- Configurable connection timeouts
- Connection source validation

#### Data Security
- No sensitive data logging in production
- Secure configuration storage
- Input validation for all external data

### Extension Points

The architecture is designed for extensibility:

#### Adding New Analyzers
1. Create new service following the pattern (`{analyzer}_service.rs`)
2. Implement protocol-specific parsing
3. Add command handlers for configuration
4. Extend app state management
5. Add frontend integration

#### Adding New Protocols
1. Create protocol module (`src-tauri/src/protocol/{protocol}_parser.rs`)
2. Implement protocol-specific state machine
3. Add frame/message parsing logic
4. Extend analyzer model with protocol support

### Development Environment

#### Required Tools
- Rust (latest stable)
- Tauri CLI
- Node.js/npm (for frontend)
- SQLite (for database)

#### Project Structure
```
nramh-lis-2/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── api/         # Tauri commands
│   │   ├── models/      # Data models
│   │   ├── services/    # Business logic
│   │   └── protocol/    # Protocol implementations (new)
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # Frontend source
├── docs/                # Documentation
└── package.json
```

This implementation guide provides the necessary context for understanding the existing architecture and implementing the BF-6900 HL7 integration following established patterns and best practices.