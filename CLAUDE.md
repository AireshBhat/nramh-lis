# CLAUDE Assistant Instructions

## Project: BF-6500 Hematology Analyzer HL7 Integration

### Overview
This file contains step-by-step instructions for Claude to help complete the BF-6500 Hematology analyzer integration with HL7 protocol support. The implementation follows the existing Meril AutoQuant ASTM pattern.

### Reference Files
Before starting any task, always read these files for context:
- `docs/status.md` - Current implementation status and detailed task breakdown
- `docs/implementation.md` - Project architecture and implementation context
- `docs/BF-6500 LIS.pdf` - BF-6500 specific HL7 requirements (extract details as needed)
- `src-tauri/src/services/autoquant_meril.rs` - Reference implementation pattern
- `src-tauri/src/api/commands/meril_handler.rs` - Reference command pattern
- `src-tauri/src/app_state.rs` - App state management pattern

### Development Guidelines

#### Code Style
- Follow existing Rust conventions in the codebase
- Use the same async/await patterns as autoquant_meril.rs
- Maintain consistent error handling patterns
- Add comprehensive logging with appropriate levels
- Include unit tests for all new functionality

#### Documentation
- Update `docs/status.md` after completing each subtask
- Add inline code documentation following existing patterns
- Document any deviations from the original plan

#### Testing
- Create unit tests for all parsing functions
- Test error conditions and edge cases
- Validate against mock HL7 messages
- Ensure thread safety in concurrent scenarios

---

## Phase 1: Core Infrastructure Setup

### Task 1.1: Create HL7 Protocol Foundation

**Before starting:**
1. Read `docs/BF-6500 LIS.pdf` to extract specific HL7 details
2. Review ASTM implementation in `autoquant_meril.rs` for patterns
3. Check existing protocol constants and structures

**Step-by-step instructions:**

#### Subtask 1.1.1: Create protocol directory structure
```bash
# Commands to run:
mkdir -p src-tauri/src/protocol
```

Create `src-tauri/src/protocol/mod.rs`:
```rust
pub mod hl7_parser;

pub use hl7_parser::*;
```

#### Subtask 1.1.2: Implement HL7 v2.4 parsing core
Create `src-tauri/src/protocol/hl7_parser.rs` with:

1. **HL7 Constants**: Define HL7/MLLP control characters
```rust
// MLLP Protocol Constants
const MLLP_START_BLOCK: u8 = 0x0B; // VT - Vertical Tab
const MLLP_END_BLOCK: u8 = 0x1C;   // FS - File Separator  
const MLLP_CARRIAGE_RETURN: u8 = 0x0D; // CR - Carriage Return
```

2. **HL7 Message Structure**: Define enums and structs for HL7 messages
3. **Field Separators**: Handle pipe-delimited parsing
4. **Segment Parsing**: Parse MSH, PID, PV1, OBR, OBX segments

#### Subtask 1.1.3: Implement MLLP frame handling
Add functions for:
- `extract_mllp_message()` - Extract message from MLLP frame
- `create_mllp_frame()` - Wrap message in MLLP framing
- `validate_mllp_frame()` - Validate frame structure

#### Subtask 1.1.4: Add HL7 acknowledgment
Implement:
- `create_ack_message()` - Generate ACK response
- `create_nak_message()` - Generate NAK response
- `parse_acknowledgment()` - Parse incoming ACK/NAK

#### Subtask 1.1.5: Create segment parsers
Implement specific parsers:
- `parse_msh_segment()` - Message header
- `parse_pid_segment()` - Patient identification
- `parse_obr_segment()` - Observation request
- `parse_obx_segment()` - Observation result

#### Subtask 1.1.6: Add unit tests
Create comprehensive tests in the same file:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Test all parsing functions with real HL7 samples
}
```

### Task 1.2: Define BF-6500 Data Models

#### Subtask 1.2.1: Extend Protocol enum
Update `src-tauri/src/models/analyzer.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Protocol {
    Astm,
    HL7_V24,  // Add this variant
}
```

#### Subtask 1.2.2: Create BF6500Event enum
In `src-tauri/src/models/hematology.rs` (new file):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BF6500Event {
    AnalyzerConnected { analyzer_id: String, remote_addr: String, timestamp: DateTime<Utc> },
    HL7MessageReceived { analyzer_id: String, message_type: String, raw_data: String, timestamp: DateTime<Utc> },
    HematologyResultProcessed { analyzer_id: String, patient_data: Option<PatientData>, test_results: Vec<HematologyResult>, timestamp: DateTime<Utc> },
    AnalyzerStatusUpdated { analyzer_id: String, status: AnalyzerStatus, timestamp: DateTime<Utc> },
    AnalyzerDisconnected { analyzer_id: String, timestamp: DateTime<Utc> },
    Error { analyzer_id: String, error: String, timestamp: DateTime<Utc> },
}
```

#### Subtask 1.2.3: Define HematologyResult struct
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematologyResult {
    pub id: String,
    pub parameter: String,    // WBC, RBC, HGB, etc.
    pub value: String,
    pub units: Option<String>,
    pub reference_range: Option<String>,
    pub flags: Vec<String>,   // H, L, A, etc.
    pub status: String,       // F=Final, P=Preliminary
    pub completed_date_time: Option<DateTime<Utc>>,
    pub analyzer_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### Subtask 1.2.4: Create HL7Settings struct
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HL7Settings {
    pub mllp_enabled: bool,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
    pub encoding: String,
    pub supported_message_types: Vec<String>,
}
```

#### Subtask 1.2.5: Update mod.rs files
Update `src-tauri/src/models/mod.rs` to export new hematology module.

---

## Phase 2: Service Implementation

### Task 2.1: Create BF-6500 Service

**Before starting:**
1. Study the complete `autoquant_meril.rs` implementation
2. Understand the event-driven architecture pattern
3. Note the connection management approach

#### Subtask 2.1.1: Create bf6500_service.rs skeleton
Create `src-tauri/src/services/bf6500_service.rs` following `autoquant_meril.rs` structure:

1. **Copy the overall structure** from AutoQuantMerilService
2. **Rename appropriately** to BF6500Service
3. **Replace ASTM constants** with HL7/MLLP constants
4. **Update event types** to use BF6500Event
5. **Modify connection state** for HL7 MLLP protocol

#### Subtask 2.1.2: Implement TCP listener
Adapt the existing TCP listener pattern:
```rust
async fn handle_connections_loop(
    listener: Arc<Mutex<Option<TcpListener>>>,
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    is_running: Arc<RwLock<bool>>,
    event_sender: mpsc::Sender<BF6500Event>,
    analyzer_id: String,
) {
    // Follow autoquant_meril.rs pattern but adapt for HL7
}
```

#### Subtask 2.1.3: Add HL7 connection state management
Create HL7-specific connection states:
```rust
#[derive(Debug, Clone)]
pub enum HL7ConnectionState {
    WaitingForMessage,    // Waiting for MLLP start
    ReadingMessage,       // Reading HL7 message content
    WaitingForEndBlock,   // Waiting for MLLP end sequence
    ProcessingMessage,    // Processing complete HL7 message
    SendingAck,          // Sending acknowledgment
    Complete,            // Message processing complete
}
```

#### Subtask 2.1.4: Create HL7 message processing pipeline
Implement `process_hl7_data()` function similar to `process_astm_data()`:
```rust
async fn process_hl7_data(
    connection: &mut Connection,
    data: &[u8],
    event_sender: &mpsc::Sender<BF6500Event>,
) -> Result<(), String> {
    // Process MLLP frames and extract HL7 messages
    // Parse HL7 segments
    // Extract hematology results
    // Send acknowledgment
    // Emit events
}
```

#### Subtask 2.1.5: Implement event emission
Follow the existing pattern but emit BF6500Event types.

#### Subtask 2.1.6: Add service lifecycle management
Implement start() and stop() methods following the existing pattern.

### Task 2.2: Configuration Management

#### Subtask 2.2.1: Create bf6500_handler.rs
Create `src-tauri/src/api/commands/bf6500_handler.rs` based on `meril_handler.rs`:

1. **Copy the structure** from meril_handler.rs
2. **Rename functions** to bf6500_* variants
3. **Update validation** for HL7 protocol
4. **Change store name** to "bf6500.json"

#### Subtask 2.2.2: Implement HL7 configuration validation
Create `validate_bf6500_config()` function:
```rust
fn validate_bf6500_config(analyzer: &Analyzer) -> Result<(), String> {
    // Validate HL7 protocol is selected
    // Validate IP/port configuration
    // Validate HL7-specific settings
    Ok(())
}
```

#### Subtask 2.2.3: Add Tauri commands
Implement these commands following the meril pattern:
- `fetch_bf6500_config()`
- `update_bf6500_config()`
- `start_bf6500_service()`
- `stop_bf6500_service()`
- `get_bf6500_service_status()`

#### Subtask 2.2.4: Create status monitoring
Implement status monitoring similar to Meril service.

#### Subtask 2.2.5: Implement configuration persistence
Save/load configuration to "bf6500.json" store.

---

## Phase 3: Protocol Implementation

### Task 3.1: HL7 Message Processing

#### Subtask 3.1.1: Implement MLLP framing
In the HL7 parser, implement complete MLLP handling:
```rust
fn extract_mllp_message(data: &[u8]) -> Result<Vec<u8>, String> {
    // Find VT (0x0B) start
    // Extract message until FS CR (0x1C 0x0D)
    // Return message content
}
```

#### Subtask 3.1.2: Create HL7 segment parser
Implement robust segment parsing:
```rust
fn parse_hl7_message(message: &str) -> Result<HL7Message, String> {
    // Split by \r (segment separator)
    // Parse each segment by | (field separator)
    // Handle component separators (^, &, ~)
    // Return structured HL7Message
}
```

#### Subtask 3.1.3: Implement ACK/NAK responses
```rust
fn create_hl7_acknowledgment(original_message: &HL7Message, ack_code: &str) -> String {
    // Create MSH segment with ACK message type
    // Include MSA segment with acknowledgment code
    // Return properly formatted HL7 ACK
}
```

#### Subtask 3.1.4: Add hematology result extraction
Implement OBX segment processing for hematology parameters:
```rust
fn extract_hematology_results(obx_segments: &[HL7Segment]) -> Vec<HematologyResult> {
    // Parse each OBX segment
    // Extract test name, value, units, reference range, flags
    // Map to HematologyResult structs
}
```

#### Subtask 3.1.5: Handle message types
Support ORU^R01 and OUL^R21 message types specifically.

#### Subtask 3.1.6: Implement error handling
Add comprehensive error handling and logging throughout.

### Task 3.2: Connection Handling

#### Subtask 3.2.1: Implement HL7 handshake
Adapt the connection handling for HL7 MLLP protocol requirements.

#### Subtask 3.2.2: Handle MLLP protocol
Ensure proper MLLP frame handling with timeout management.

#### Subtask 3.2.3: Manage timeouts and retries
Implement configurable timeouts and retry logic.

#### Subtask 3.2.4: Add connection monitoring
Implement health checks and connection status monitoring.

#### Subtask 3.2.5: Implement cleanup
Ensure proper resource cleanup on connection close.

---

## Phase 4: Integration

### Task 4.1: App State Integration

#### Subtask 4.1.1: Extend AppState
Update `src-tauri/src/app_state.rs`:

1. **Add BF6500 service field** to AppState struct
2. **Create BF6500 service** in new() method
3. **Add event handling** for BF6500Event types
4. **Implement service lifecycle** methods

#### Subtask 4.1.2: Add service initialization
Add BF6500 service initialization in app startup.

#### Subtask 4.1.3: Implement lifecycle management
Add start/stop methods for BF6500 service.

#### Subtask 4.1.4: Add auto-start support
Support activate_on_start configuration.

#### Subtask 4.1.5: Create default configuration
Implement `create_default_bf6500_analyzer()` function.

### Task 4.2: Frontend Events

#### Subtask 4.2.1: Define event types
Add BF6500 event emission in app_state.rs event handler.

#### Subtask 4.2.2: Implement event emission
Follow the existing pattern for emitting events to frontend.

#### Subtask 4.2.3: Add event handling
Ensure events are properly routed through app state.

#### Subtask 4.2.4: Test event flow
Verify events reach the frontend correctly.

---

## Phase 5: Testing & Validation

### Task 5.1: Unit Testing

#### Subtask 5.1.1: Create HL7 parser tests
Add comprehensive tests for all HL7 parsing functions.

#### Subtask 5.1.2: Test MLLP handling
Test MLLP frame extraction and creation.

#### Subtask 5.1.3: Test result parsing
Test hematology result extraction with sample data.

#### Subtask 5.1.4: Test configuration validation
Test all validation functions.

#### Subtask 5.1.5: Test service lifecycle
Test service start/stop operations.

### Task 5.2: Integration Testing

#### Subtask 5.2.1: Create mock HL7 server
Build a simple HL7 message sender for testing.

#### Subtask 5.2.2: Test end-to-end flow
Test complete message flow from connection to result processing.

#### Subtask 5.2.3: Test error scenarios
Test various error conditions and recovery.

#### Subtask 5.2.4: Test configuration persistence
Verify configuration save/load functionality.

#### Subtask 5.2.5: Performance testing
Test with multiple connections and high message volume.

---

## Execution Guidelines

### When Starting Any Task:
1. **Read the status.md file** to understand current progress
2. **Review relevant reference files** mentioned above
3. **Understand the existing pattern** from Meril implementation
4. **Plan the specific implementation** for HL7/BF-6500
5. **Write the code** following existing conventions
6. **Test thoroughly** with appropriate test cases
7. **Update status.md** with completion status
8. **Document any issues or deviations**

### When Completing Each Subtask:
1. **Mark the subtask as complete** in status.md
2. **Run any tests** to ensure functionality
3. **Check for compilation errors**
4. **Verify integration** with existing code
5. **Document any changes** made to the original plan

### Error Handling:
- If you encounter issues with the PDF, ask the user for specific details
- If existing code doesn't match expectations, analyze and adapt
- Always maintain backward compatibility with existing Meril functionality
- Document any architectural decisions or trade-offs

### Testing Strategy:
- Write unit tests for all new parsing functions
- Create integration tests with mock data
- Test error conditions thoroughly
- Validate thread safety in concurrent scenarios
- Test configuration edge cases

### Code Quality:
- Follow existing Rust conventions in the codebase
- Use the same error handling patterns
- Maintain consistent logging levels
- Add comprehensive documentation
- Ensure proper resource cleanup

---

## Current Task to Start:
Based on status.md, begin with **Phase 1, Task 1.1, Subtask 1.1.1** - Create protocol directory structure.

### Quick Start Command:
When the user asks you to start implementation, begin with:
1. Read docs/status.md to see current status
2. Start with the first uncompleted task
3. Follow the step-by-step instructions above
4. Update status.md when each subtask is complete

Remember: This is a complex integration project. Take it one step at a time, follow the established patterns, and maintain high code quality throughout the implementation.