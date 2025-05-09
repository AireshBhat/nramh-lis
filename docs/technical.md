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
- **TCP/IP Communication**: tokio/async-std
- **File Storage**: Native file system operations
- **Authentication**: JWT + Argon2 password hashing
- **Logging**: tracing/log crates
- **Backend Folder**: src-tauri

## 3. Core Modules

### 3.1 Frontend Modules

#### User Interface
- **Dashboard**: Main operational view of the system
- **Patient Management**: Interface for managing patient records
- **Test Management**: Interface for managing test orders and results
- **Configuration**: System configuration and device settings
- **Reporting**: Test result reporting and analytics

#### UI Components
- Reusable component library adhering to design system
- Form elements with validation
- Data tables with sorting, filtering, and pagination
- Charts and visualization components
- Notification system

#### State Management
- Global application state
- User authentication state
- Form state management
- Query cache and synchronization

#### Custom Hooks
- Communication hooks
- Form handling hooks
- Authentication hooks
- Data fetching hooks

#### API Client
- Request/response handling
- Error handling
- Authentication header management
- Request cancellation

### 3.2 Tauri Application Layer

#### Commands
- Exposed Rust functions for frontend consumption
- Authentication and permission validation
- Data transformation between frontend and backend

#### Event System
- Bi-directional event communication
- Real-time updates from devices
- System notifications

#### IPC Bridge
- Message serialization/deserialization
- Error handling and recovery
- Performance optimization

#### Window Management
- Multi-window support
- Modal dialogs
- System tray integration

### 3.3 Backend Modules

#### Handler Layer
- **LIS Communication Handler**: Manages ASTM protocol communications
- **Device Handler**: Manages connections to laboratory analyzers
- **Data Handler**: Processes and validates incoming/outgoing data
- **User Handler**: Manages user authentication and permissions
- **Configuration Handler**: Manages system configuration

#### Service Layer
- **ASTM Protocol Service**: Implements ASTM E1381 and E1394 standards
- **Patient Service**: Manages patient information
- **Result Service**: Processes and stores test results
- **Authentication Service**: Handles user authentication and session management
- **Reporting Service**: Generates reports from test results
- **Validation Service**: Validates data against business rules

#### Storage Layer
- **Database Adapter**: Abstracts database operations
- **File Storage**: Manages report files and exports
- **Connection Pool**: Manages database connections
- **Migration Manager**: Handles database schema changes
- **Cache Manager**: Optimizes performance for frequently accessed data

### 3.4 External Systems Integration

- **AutoQuant Analyzers**: Direct communication with laboratory equipment
- **Hospital Information Systems**: Data exchange with hospital systems
- **Other Laboratory Systems**: Integration with complementary laboratory systems

## 4. Communication Protocols

### 4.1 ASTM Protocol Implementation
- **Physical Layer**: TCP/IP communication
- **Data Link Layer**: Frame-based communication with checksums
- **Application Layer**: Record transmission protocol
  - Message Header Record (H)
  - Patient Information Record (P)
  - Test Order Record (O)
  - Result Record (R)
  - Comment Record (C)
  - Request Information Record (Q)
  - Message Terminator Record (L)

### 4.2 Communication Flow
- Establishment Phase (ENQ, ACK, NAK)
- Transfer Phase (data frames with STX, ETX/ETB, checksum)
- Termination Phase (EOT)

## 5. Data Models

### 5.1 Patient Model
- Patient identification
- Demographics
- Contact information
- Medical information

### 5.2 Sample Model
- Sample identification
- Collection information
- Type and container
- Status and tracking

### 5.3 Test Order Model
- Test identification
- Priority and scheduling
- Specimen information
- Ordering provider

### 5.4 Result Model
- Test results with units
- Reference ranges
- Flags for abnormal results
- Testing metadata

### 5.5 User Model
- Authentication credentials
- Permissions and roles
- Profile information
- Activity logging

## 6. Development Workflow

### 6.1 Development Environment
- Local development using Vite dev server
- Hot module replacement for frontend
- Cargo watch for auto-recompiling Rust code
- Docker containerization for consistent environments

### 6.2 Code Organization
- Feature-based organization
- Shared utility libraries
- Clear separation between layers
- Comprehensive test coverage

### 6.3 Version Control
- Git-based workflow
- Feature branches
- Pull request reviews
- Semantic versioning

### 6.4 Testing Strategy
- Unit testing for business logic
- Integration testing for communication protocols
- End-to-end testing for critical workflows
- Mocking external systems

### 6.5 Deployment
- Tauri bundling for platform-specific executables
- Automatic updates mechanism
- Database migration system
- Configuration backup and restore

## 7. Security Considerations

- Secure storage of patient data
- Encryption of communications
- Authentication and authorization
- Audit logging
- Regular security updates

## 8. Performance Considerations

- Efficient database queries
- Connection pooling
- Caching strategy
- Optimized UI rendering
- Background processing for intensive tasks

## 9. Compliance and Standards

- HIPAA compliance for patient data
- HL7/FHIR compatibility options
- ASTM E1381 and E1394 standards
- Regional regulatory requirements

## 10. Future Extensibility

- Plugin architecture
- API for third-party integrations
- Configuration-driven features
- Support for additional analyzer protocols