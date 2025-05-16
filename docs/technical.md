# Laboratory Interfacing Software (LIS) - Technical Specification

## 1. Overview

This document outlines the technical specifications for a Laboratory Interfacing Software (LIS) system designed to facilitate communication between laboratory analyzers (specifically the Meril AutoQuant series) and hospital information systems using the ASTM protocol. The system features a modern desktop application architecture with a clear separation of concerns and robust communication capabilities.

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
- **ORM**: SQLx (Implemented)
- **TCP/IP Communication**: tokio for async I/O (Implemented)
- **File Storage**: Native file system operations
- **Logging**: tracing/log crates
- **Backend Folder**: src-tauri

## 3. Core Modules

### 3.1 Frontend Modules

#### User Interface
- **Dashboard**: Operational view showing connection status and received test results
- **Result View**: Display of recently received test results
- **Configuration View**: Settings for connections and system parameters

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
- TCP server status commands (Implemented)
- Test result retrieval commands (Implemented)
- Configuration commands for TCP settings

#### Event System
- Connection status events
- New result notification events
- Error notification events

#### IPC Bridge
- Message serialization/deserialization (Implemented)
- Event forwarding between Rust and frontend

### 3.3 Backend Modules

#### Application Startup
- Configuration loading
- Database initialization
- Services initialization

#### Core Services
- **Machine Service**: Manages communication with medical analyzers (In Progress)
- **Hospital Information Bridge**: Handles communication with hospital systems (Planned)

#### Machine Service Internal
- **Server Listener**: Manages TCP connections from analyzers
- **Message Handler**: Processes incoming messages
- **Patient Service**: Manages patient data
- **Result Service**: Processes and stores test results
- **Event Emitter**: Triggers events for frontend and HIS integration

#### Handler Layer
- **Configuration Handler**: Manages system settings (Planned)
- **User Handler**: Manages user authentication and permissions (Planned)
- **UI Handler**: Handles UI-related operations (Planned)

#### Protocol Layer
- **ASTM Protocol Service**: Implements ASTM E1381 and E1394 standards
  - Supports Meril AutoQuant series analyzers
  - Handles frame parsing and validation
  - Processes ASTM records (H, P, O, R, C, Q, L)
- **HL7 Service**: For future HL7 protocol support (Planned)
- **FHIR Service**: For future FHIR protocol support (Planned)

#### Storage Layer
- **Database Adapter**: Abstraction for storing test results (Implemented)
- **File Storage**: For storing logs and exports (Planned)
- **Connection Pool**: Management for database operations (Implemented)
- **Migration Manager**: Database schema versioning (Implemented)
- **Cache Manager**: Performance optimization for frequently accessed data (Planned)

### 3.4 External Systems Integration
- **Medical Analyzers**: Communication with laboratory equipment
  - Meril AutoQuant series (In Progress)
- **Hospital Information Systems**: Integration with hospital systems (Planned)
- **Laboratory Information Systems**: Integration with lab systems (Planned)

## 4. Communication Protocols

### 4.1 ASTM Protocol Implementation (IMPLEMENTED)
- **Physical Layer**: TCP/IP communication (IMPLEMENTED)
  - Socket management with tokio (IMPLEMENTED)
  - Connection handling and timeout management (IMPLEMENTED)
  - Error recovery mechanisms (IMPLEMENTED)

- **Data Link Layer**: Frame-based communication with checksums (IMPLEMENTED)
  - ENQ-ACK handshake implementation (IMPLEMENTED)
  - Frame construction and parsing (IMPLEMENTED)
  - Checksum calculation and validation (IMPLEMENTED)
  - ACK, NAK, EOT handling (IMPLEMENTED)

- **Application Layer**: Record transmission protocol
  - Message Header Record (H)
  - Patient Information Record (P)
  - Test Order Record (O)
  - Result Record (R)
  - Comment Record (C)
  - Request Information Record (Q)
  - Message Terminator Record (L)

### 4.2 Meril AutoQuant Communication Flow
The system implements the specific communication flow required by Meril AutoQuant analyzers:

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

## 5. Data Models

### 5.1 Patient Model (PLANNED)
- Patient identification (ID, name)
- Demographics (age, gender, DOB)
- Contact information
- Relationships to other records

### 5.2 Result Model (PLANNED)
- Test results with units
- Reference ranges
- Flags for abnormal results
- Testing metadata
- Analyzer identification
- Timestamps for collection, testing, and reporting

### 5.3 Analyzer Model (PLANNED)
- Analyzer configuration and status
- Connection settings
- Device identification
- Protocol settings

### 5.4 Upload Status Model (PLANNED)
- Result upload tracking
- Status monitoring
- Response handling
- Error tracking

## 6. Development Roadmap

### 6.1 Phase 1: Core Infrastructure (COMPLETED)
- Database setup
- ASTM protocol implementation
- Basic TCP communication

### 6.2 Phase 2: Core Services (IN PROGRESS)
- Patient service implementation
- Test result service implementation
- Machine service for Meril AutoQuant analyzers

### 6.3 Phase 3: Integration Layer (PLANNED)
- Hospital bridge service
- Result upload functionality
- Frontend integration

### 6.4 Phase 4: User Interface (PLANNED)
- Dashboard development
- Result viewer implementation
- Configuration interface

## 7. Security Considerations

- Secure storage of patient data
- Input validation for all received ASTM messages
- Audit logging of communication events
- User authentication and authorization
- Data encryption where appropriate

## 8. Future Expansions

After implementing the core functionality, the system will be expanded to include:
- Support for additional analyzer protocols
- Advanced patient management
- Comprehensive reporting
- Enhanced hospital information system integration
- Mobile access capabilities
- Analytics and trend analysis