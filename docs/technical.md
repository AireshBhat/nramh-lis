# Laboratory Interfacing Software (LIS) - Technical Specification

## 1. Overview

This document outlines the technical specifications for a Laboratory Interfacing Software (LIS) system designed to facilitate communication between laboratory analyzers (specifically AutoQuant series) and hospital information systems using the ASTM protocol. The system features a modern desktop application architecture with a clear separation of concerns and robust communication capabilities.

## 2. Technology Stack

### Frontend
- **Build System**: Vite
- **Framework**: React
- **Routing**: TanStack React Router
- **State Management**: React Query + Context API
- **UI Components**: Tailwind CSS + Headless UI
- **API Client**: Axios/Fetch with custom wrapper
- **Frontend Folder**: src

### Desktop Application Layer
- **Framework**: Tauri (Rust + Web Tech)
- **IPC**: Tauri Commands and Events API
- **Window Management**: Tauri Window API

### Backend (Rust)
- **Language**: Rust
- **Database**: SQLite with optional PostgreSQL
- **ORM**: SQLx
- **TCP/IP Communication**: tokio for async I/O
- **File Storage**: Native file system operations
- **Logging**: tracing/log crates
- **Backend Folder**: src-tauri

## 3. Core Modules

### 3.1 Frontend Modules

#### User Interface
- **Dashboard**: Simple operational view showing connection status and received test results
- **Result View**: Display of recently received test results

#### UI Components
- Reusable component library adhering to design system
- Data tables with sorting and filtering for results
- Status indicators for TCP connections
- Notification components for system events

#### State Management
- Global application state
- Query cache for test results and connection status

#### Custom Hooks
- Communication hooks for Tauri command integration
- Data fetching hooks for test results

#### API Client
- Tauri command wrappers
- Error handling

### 3.2 Tauri Application Layer

#### Commands
- TCP server status commands
- Test result retrieval commands
- Configuration commands for TCP settings

#### Event System
- Connection status events
- New result notification events
- Error notification events

#### IPC Bridge
- Message serialization/deserialization
- Event forwarding between Rust and frontend

### 3.3 Backend Modules

#### Handler Layer
- **TCP Communication Handler**: Manages TCP server and client connections (IMMEDIATE FOCUS)
- **ASTM Protocol Handler**: Handles ASTM protocol message parsing and validation (IMMEDIATE FOCUS)
- **Data Handler**: Processes and validates incoming result data (IMMEDIATE FOCUS)

#### Service Layer
- **ASTM Protocol Service**: Implements ASTM E1381 and E1394 standards (IMMEDIATE FOCUS)
- **Result Service**: Processes and stores test results (IMMEDIATE FOCUS)

#### Storage Layer
- **Database Adapter**: Simple abstraction for storing test results
- **Connection Pool**: Basic connection management for database operations

### 3.4 External Systems Integration
- **AutoQuant Analyzers**: TCP/IP communication with laboratory equipment (IMMEDIATE FOCUS)

## 4. Communication Protocols

### 4.1 ASTM Protocol Implementation (IMMEDIATE FOCUS)
- **Physical Layer**: TCP/IP communication
  - Socket management with tokio
  - Connection handling and timeout management
  - Error recovery mechanisms

- **Data Link Layer**: Frame-based communication with checksums
  - ENQ-ACK handshake implementation
  - Frame construction and parsing
  - Checksum calculation and validation
  - ACK, NAK, EOT handling

- **Application Layer**: Record transmission protocol
  - Message Header Record (H)
  - Patient Information Record (P)
  - Result Record (R)
  - Message Terminator Record (L)

### 4.2 Communication Flow (IMMEDIATE FOCUS)
The core TCP/IP communication flow with AutoQuant analyzers follows this sequence:

1. **Establishment Phase**
   - LIS initializes TCP server and listens on configured port
   - Analyzer connects to LIS TCP server
   - Analyzer sends ENQ to initiate communication
   - LIS acknowledges with ACK

2. **Transfer Phase**
   - Analyzer sends data frames (each containing ASTM records)
   - LIS validates each frame (checksum verification)
   - LIS responds with ACK for valid frames or NAK for invalid frames
   - Analyzer retransmits frames that received NAK

3. **Termination Phase**
   - Analyzer sends EOT to signal end of transmission
   - LIS processes complete message
   - Results are stored in database
   - UI is updated with new results
   - Connection is closed or returned to waiting state

This flow is documented in detail in the `docs/important-flows/tcp-communication-meril.mermaid` diagram.

## 5. Data Models

### 5.1 Patient Model (Simplified for Initial Implementation)
- Patient identification
- Basic demographics

### 5.2 Result Model (IMMEDIATE FOCUS)
- Test results with units
- Reference ranges
- Flags for abnormal results
- Testing metadata
- Analyzer identification

## 6. Development Workflow

### 6.1 Development Environment
- Local development using Vite dev server
- Hot module replacement for frontend
- Cargo watch for auto-recompiling Rust code

### 6.2 Code Organization
- Feature-based organization focusing first on TCP communication
- Shared utility libraries for ASTM protocol handling

### 6.3 Testing Strategy
- Unit testing for ASTM protocol parsing
- Integration testing for TCP communication
- ASTM protocol simulator for end-to-end testing without physical analyzers

## 7. Initial Implementation Plan

### 7.1 Phase 1: TCP Server and Protocol Handler
- Implement basic TCP server with tokio
- Develop ASTM protocol frame parsing and validation
- Create simple data storage for received results

### 7.2 Phase 2: Frontend Integration
- Implement Tauri commands for accessing TCP server status
- Create basic UI for displaying connection status
- Implement real-time updates for received results

### 7.3 Phase 3: Testing and Validation
- Develop ASTM protocol simulator for testing
- Validate against real-world ASTM messages
- Implement error handling and recovery mechanisms

## 8. Security Considerations

- Secure storage of patient data
- Input validation for all received ASTM messages
- Audit logging of communication events

## 9. Future Expansions
After implementing the core TCP communication flow, the system will be expanded to include:

- Advanced patient management
- Comprehensive reporting
- Hospital information system integration
- Additional analyzer protocol support
- Enhanced user interface with more features