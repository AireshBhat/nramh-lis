# Lab Machine Interface System - Implementation Status & Plan

## Project Status Overview

**Current Phase**: Planning Complete ✅  
**Target**: POC in 5 hours of focused development  
**Priority**: MVP functionality first, enhancements later

## 🎯 Success Criteria (5-Hour Target)

- [x] Basic Tauri app with frontend
- [ ] Single machine connection (preferably mock/simulator)
- [ ] Simple message parsing (HL7 or ASTM)
- [ ] Basic UI to show connection status and messages
- [ ] JSON configuration loading

## Implementation Roadmap

### Phase 1: Foundation (HIGH Priority - 2 hours)

#### 1.1 Project Setup (30 minutes)
- [x] **CRITICAL** Initialize Tauri project
  ```bash
  cargo install tauri-cli
  cargo tauri init
  ```
- [x] **CRITICAL** Setup basic React frontend
- [x] **CRITICAL** Configure Tauri commands structure
- [x] **CRITICAL** Setup basic folder structure

**Files to create:**
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`
- `src/App.tsx` (basic React app)
- `src-tauri/tauri.conf.json`

#### 1.2 Core Data Models (30 minutes)
- [ ] **HIGH** Create basic data structures
  - `models/machine.rs`
  - `models/message.rs`
  - `models/config.rs`
- [ ] **HIGH** Implement serialization/deserialization
- [ ] **HIGH** Basic error types

**Priority Models:**
```rust
// Minimal for POC
pub struct MachineConfig {
    pub id: String,
    pub name: String,
    pub protocol: Protocol,
    pub transport: Transport,
}

pub struct LabMessage {
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
    pub raw_data: String,
    pub parsed: bool,
}
```

#### 1.3 Configuration System (30 minutes)
- [ ] **HIGH** JSON configuration loading
- [ ] **HIGH** Basic configuration validation
- [ ] **HIGH** Default configuration generation

**Config Structure:**
```json
{
  "machines": [
    {
      "id": "TEST-001",
      "name": "Test Machine",
      "protocol": "HL7",
      "transport": {
        "type": "TCP",
        "host": "localhost",
        "port": 9100
      }
    }
  ]
}
```

#### 1.4 Basic API Layer (30 minutes)
- [ ] **HIGH** Create Tauri command handlers
  - `get_config()`
  - `add_machine()`
  - `get_machines()`
  - `connect_machine()`

### Phase 2: Core Communication (HIGH Priority - 1.5 hours)

#### 2.1 Protocol Foundation (45 minutes)
- [ ] **HIGH** Basic HL7 message parser (simpler than ASTM)
- [ ] **HIGH** Message validation structure
- [ ] **MEDIUM** Simple ASTM parser (if time permits)

**Focus on HL7 first** - simpler structure:
```
MSH|^~\&|Device||||20240101120000||ORU^R01|123|P|2.4
PID||12345|||John^Doe
OBX|1|NM|GLU||150|mg/dL
```

#### 2.2 Communication Manager (45 minutes)
- [ ] **HIGH** TCP socket connection (easier to test)
- [ ] **MEDIUM** Serial port connection (if time permits)
- [ ] **HIGH** Basic connection state management
- [ ] **HIGH** Message receiving and parsing

**MVP Implementation:**
```rust
pub struct SimpleConnection {
    pub machine_id: String,
    pub status: ConnectionStatus,
    // Start with TCP for easier testing
}
```

### Phase 3: UI Integration (HIGH Priority - 1 hour)

#### 3.1 Basic React Components (30 minutes)
- [ ] **HIGH** Machine list component
- [ ] **HIGH** Connection status indicator
- [ ] **HIGH** Message log viewer
- [ ] **HIGH** Basic controls (connect/disconnect)

#### 3.2 Tauri Integration (30 minutes)
- [ ] **HIGH** Command invocation from frontend
- [ ] **HIGH** Real-time status updates
- [ ] **HIGH** Error display and handling

### Phase 4: Testing & Demo (MEDIUM Priority - 30 minutes)

#### 4.1 Mock/Simulator (20 minutes)
- [ ] **HIGH** Create simple TCP server that sends HL7 messages
- [ ] **HIGH** Test data for demo

#### 4.2 Integration Testing (10 minutes)
- [ ] **HIGH** End-to-end test: config → connect → receive → display

## Detailed Task Breakdown

### Week 1: POC Development (40 hours total)

#### Day 1 (8 hours) - Foundation
- **Hours 1-2**: Project setup and basic structure
- **Hours 3-4**: Data models and configuration system
- **Hours 5-6**: Basic Tauri commands and API layer
- **Hours 7-8**: Initial frontend structure

#### Day 2 (8 hours) - Core Communication
- **Hours 1-3**: HL7 protocol parsing implementation
- **Hours 4-6**: TCP communication manager
- **Hours 7-8**: Connection state management

#### Day 3 (8 hours) - UI and Integration
- **Hours 1-4**: React components and UI
- **Hours 5-6**: Tauri-React integration
- **Hours 7-8**: Basic testing and debugging

#### Day 4 (8 hours) - Testing and Polish
- **Hours 1-2**: Mock server for testing
- **Hours 3-4**: End-to-end testing
- **Hours 5-6**: Error handling improvements
- **Hours 7-8**: Documentation and demo prep

#### Day 5 (8 hours) - Enhancement and ASTM
- **Hours 1-4**: ASTM protocol implementation
- **Hours 5-6**: Serial communication
- **Hours 7-8**: Additional features and polish

### Implementation Priority Matrix

#### CRITICAL (Must have for POC)
1. ✅ Basic Tauri app structure
2. ⏳ Single machine configuration
3. ⏳ TCP connection to mock server
4. ⏳ Basic HL7 message parsing
5. ⏳ Simple UI to show status

#### HIGH (Important for demo)
1. ⏳ Multiple machine support
2. ⏳ Message history display
3. ⏳ Error handling and display
4. ⏳ Configuration editing via UI
5. ⏳ Real-time status updates

#### MEDIUM (Nice to have)
1. ⏳ ASTM protocol support
2. ⏳ Serial port communication
3. ⏳ Advanced message validation
4. ⏳ Export functionality
5. ⏳ System metrics

#### LOW (Future enhancements)
1. ⏳ HIS integration
2. ⏳ Advanced error recovery
3. ⏳ Performance optimizations
4. ⏳ Comprehensive testing
5. ⏳ Production deployment features

## Risk Assessment & Mitigation

### High Risk Items
1. **Serial port communication complexity**
   - *Mitigation*: Start with TCP, add serial later
2. **Protocol parsing complexity**
   - *Mitigation*: Start with HL7 (simpler), focus on basic parsing
3. **Real lab machine integration**
   - *Mitigation*: Use mock servers for initial development

### Medium Risk Items
1. **Tauri learning curve**
   - *Mitigation*: Focus on basic commands first
2. **Cross-platform compatibility**
   - *Mitigation*: Develop on primary target platform first

## Development Environment Setup

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js and npm
# Install Tauri CLI
cargo install tauri-cli

# Install additional dependencies
cargo install tokio-serial  # For serial communication
```

### Project Structure
```
lab-machine-interface/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── api/            # Tauri commands
│   │   ├── core/           # Business logic
│   │   ├── protocol/       # Protocol handlers
│   │   └── models/         # Data structures
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # React frontend
│   ├── components/
│   ├── hooks/
│   ├── services/
│   └── App.tsx
├── config/                 # Configuration files
│   ├── machines.json
│   └── system.json
└── test-data/             # Mock data for testing
    └── sample-messages.json
```

## Testing Strategy for POC

### Unit Tests (Minimal for POC)
- [ ] Configuration loading/saving
- [ ] Basic message parsing
- [ ] Connection state management

### Integration Tests
- [ ] **CRITICAL** End-to-end: UI → Backend → Mock machine
- [ ] Configuration changes reflected in UI
- [ ] Error handling and display

### Manual Testing Checklist
- [ ] App starts without errors
- [ ] Can add/remove machine configuration
- [ ] Can connect to mock TCP server
- [ ] Messages displayed in UI
- [ ] Errors handled gracefully
- [ ] Configuration persists between restarts

## Success Metrics

### POC Success (5-hour milestone)
- ✅ Application launches
- ⏳ Single machine connection works
- ⏳ Basic message parsing functional
- ⏳ UI shows connection status
- ⏳ Can receive and display messages

### Demo Ready (Day 3 milestone)
- ⏳ Multiple machine support
- ⏳ Real-time status updates
- ⏳ Message history and filtering
- ⏳ Error handling and recovery
- ⏳ Professional UI/UX

### Production Ready (Week 2+ milestone)
- ⏳ Comprehensive error handling
- ⏳ Full protocol compliance
- ⏳ Performance optimization
- ⏳ Security implementation
- ⏳ Comprehensive testing

## Next Steps

1. **Immediate (Today)**:
   - Initialize Tauri project
   - Setup basic folder structure
   - Create core data models

2. **This Week**:
   - Complete POC implementation
   - Create mock testing environment
   - Basic UI implementation

3. **Next Week**:
   - Real machine integration
   - Advanced features
   - Production readiness

## Notes and Considerations

- **Focus on HL7 first** - more standardized and easier to parse
- **Use TCP for initial testing** - easier to debug than serial
- **Keep UI simple initially** - functional over beautiful for POC
- **Mock everything external** - don't depend on real machines for POC
- **Incremental development** - get basic end-to-end working first
- **Document assumptions** - what works, what doesn't, what's mocked