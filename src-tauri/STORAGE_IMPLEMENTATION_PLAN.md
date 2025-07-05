# Storage Implementation Plan for NRAMH LIS 2

## Overview

This document outlines the comprehensive storage implementation for the Laboratory Information System (LIS) using a trait-based repository pattern. The implementation provides a clean separation of concerns, testability, and flexibility for different storage backends.

## Architecture

### Core Components

1. **Storage Traits** (`src/storage/traits.rs`)
   - Base `Repository<T>` trait for CRUD operations
   - Specialized repository traits for each entity
   - `StorageService` trait for coordinating all repositories
   - `MachineConnector` trait for analyzer communication

2. **Repository Implementations** (`src/storage/repositories/`)
   - SQLite implementations for all repositories
   - Each repository handles its specific entity and business logic

3. **Storage Service** (`src/storage/sqlite.rs`)
   - Coordinates all repositories
   - Handles database initialization and migrations
   - Provides backup and restore functionality

## Entity Models

### 1. Analyzers
- **Purpose**: Laboratory instruments that perform tests
- **Key Features**:
  - Connection management (Serial/TCP-IP)
  - Protocol support (ASTM/HL7)
  - Status tracking (Active/Inactive/Maintenance)
  - Configuration management

### 2. Patients
- **Purpose**: Patient demographic and clinical information
- **Key Features**:
  - Comprehensive demographic data
  - Physician relationships
  - Physical attributes (height/weight)
  - Search and identification

### 3. Samples
- **Purpose**: Biological specimens for testing
- **Key Features**:
  - Container and collection information
  - Sample type classification
  - Status tracking (Pending/InProgress/Completed)
  - Position tracking in analyzers

### 4. Test Orders
- **Purpose**: Laboratory test requests
- **Key Features**:
  - Test specifications
  - Priority levels
  - Action codes (Add/New/Cancel)
  - Scheduling information

### 5. Test Results
- **Purpose**: Laboratory test results and measurements
- **Key Features**:
  - Test values and units
  - Reference ranges
  - Abnormal flags
  - Result status (Final/Preliminary/Correction)

### 6. Upload Queue
- **Purpose**: External system integration
- **Key Features**:
  - Upload status tracking
  - Retry mechanisms
  - Response handling
  - Error management

### 7. System Settings
- **Purpose**: Application configuration
- **Key Features**:
  - Key-value storage
  - Categorized settings
  - Default values
  - Runtime configuration

## Database Schema

### Tables

1. **analyzers**
   ```sql
   CREATE TABLE analyzers (
       id TEXT PRIMARY KEY,
       name TEXT NOT NULL,
       model TEXT NOT NULL,
       serial_number TEXT,
       manufacturer TEXT,
       connection_type TEXT NOT NULL,
       ip_address TEXT,
       port INTEGER,
       com_port TEXT,
       baud_rate INTEGER,
       protocol TEXT NOT NULL,
       status TEXT NOT NULL,
       activate_on_start BOOLEAN NOT NULL DEFAULT 0,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

2. **patients**
   ```sql
   CREATE TABLE patients (
       id TEXT PRIMARY KEY,
       last_name TEXT,
       first_name TEXT,
       middle_name TEXT,
       title TEXT,
       birth_date DATETIME,
       sex TEXT NOT NULL,
       street TEXT,
       city TEXT,
       state TEXT,
       zip TEXT,
       country_code TEXT,
       telephone TEXT,
       ordering_physician TEXT,
       attending_physician TEXT,
       referring_physician TEXT,
       height_value REAL,
       height_unit TEXT,
       weight_value REAL,
       weight_unit TEXT,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

3. **samples**
   ```sql
   CREATE TABLE samples (
       id TEXT PRIMARY KEY,
       container_number TEXT,
       container_type TEXT,
       collection_date_time DATETIME,
       collector_id TEXT,
       reception_date_time DATETIME,
       sample_type TEXT NOT NULL,
       status TEXT NOT NULL,
       position TEXT,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

4. **test_orders**
   ```sql
   CREATE TABLE test_orders (
       id TEXT PRIMARY KEY,
       sequence_number INTEGER NOT NULL,
       specimen_id TEXT NOT NULL,
       tests TEXT NOT NULL, -- JSON array of tests
       priority TEXT NOT NULL,
       action_code TEXT NOT NULL,
       ordering_provider TEXT,
       collection_date DATETIME,
       received_date DATETIME,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

5. **test_results**
   ```sql
   CREATE TABLE test_results (
       id TEXT PRIMARY KEY,
       test_id TEXT NOT NULL,
       sample_id TEXT NOT NULL,
       value TEXT NOT NULL,
       units TEXT,
       reference_range_lower REAL,
       reference_range_upper REAL,
       abnormal_flag TEXT,
       nature_of_abnormality TEXT,
       status TEXT NOT NULL,
       sequence_number INTEGER NOT NULL,
       instrument TEXT,
       completed_date_time DATETIME,
       analyzer_id TEXT,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

6. **upload_status**
   ```sql
   CREATE TABLE upload_status (
       id TEXT PRIMARY KEY,
       result_id TEXT NOT NULL,
       external_system_id TEXT NOT NULL,
       status TEXT NOT NULL,
       upload_date DATETIME,
       response_code TEXT,
       response_message TEXT,
       retry_count INTEGER NOT NULL DEFAULT 0,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

7. **system_settings**
   ```sql
   CREATE TABLE system_settings (
       key TEXT PRIMARY KEY,
       value TEXT NOT NULL,
       description TEXT,
       category TEXT NOT NULL,
       created_at DATETIME NOT NULL,
       updated_at DATETIME NOT NULL
   );
   ```

### Indexes

Performance indexes are created for:
- Analyzer status and serial number
- Patient name and birth date
- Sample status, type, and position
- Test order specimen and priority
- Test result sample, test ID, analyzer, and status
- Upload status result and status
- System settings category

## Repository Traits

### Base Repository
```rust
#[async_trait]
pub trait Repository<T> {
    type Error: std::error::Error + Send + Sync;
    type Id;

    async fn create(&self, entity: &T) -> Result<Self::Id, Self::Error>;
    async fn find_by_id(&self, id: Self::Id) -> Result<Option<T>, Self::Error>;
    async fn update(&self, id: Self::Id, entity: &T) -> Result<(), Self::Error>;
    async fn delete(&self, id: Self::Id) -> Result<(), Self::Error>;
    async fn list(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<T>, Self::Error>;
}
```

### Specialized Repositories

Each entity has a specialized repository with domain-specific methods:

- **AnalyzerRepository**: Status management, connection types, active analyzers
- **PatientRepository**: Search by name, birth date ranges, recent patients
- **TestResultRepository**: Patient time ranges, batch operations, abnormal results
- **SampleRepository**: Status filtering, position tracking, collection dates
- **TestOrderRepository**: Specimen linking, priority filtering, provider queries
- **UploadRepository**: Status tracking, retry management, failed uploads
- **SystemSettingsRepository**: Configuration management, categorized settings

## Usage Examples

### Initializing Storage
```rust
use crate::storage::sqlite::SqliteStorageService;
use crate::storage::db::establish_connection;

async fn setup_storage() -> Result<SqliteStorageService, Error> {
    let pool = establish_connection().await?;
    let storage = SqliteStorageService::new(pool).await?;
    storage.initialize().await?;
    Ok(storage)
}
```

### Working with Analyzers
```rust
async fn manage_analyzers(storage: &SqliteStorageService) -> Result<(), Error> {
    // Create analyzer
    let analyzer = Analyzer { /* ... */ };
    let id = storage.analyzers().create(&analyzer).await?;
    
    // Find active analyzers
    let active = storage.analyzers().find_active_analyzers().await?;
    
    // Update status
    storage.analyzers().update_status(&id, AnalyzerStatus::Maintenance).await?;
    
    Ok(())
}
```

### Working with Patients
```rust
async fn manage_patients(storage: &SqliteStorageService) -> Result<(), Error> {
    // Find or create patient
    let patient = storage.patients().find_or_create_by_identifier(&patient_data).await?;
    
    // Search by name
    let results = storage.patients().search_by_name("Smith", Some("John")).await?;
    
    // Find recent patients
    let recent = storage.patients().find_recent_patients(30).await?;
    
    Ok(())
}
```

### Working with Test Results
```rust
async fn manage_results(storage: &SqliteStorageService) -> Result<(), Error> {
    // Batch insert results
    let result_ids = storage.test_results().batch_insert(&results).await?;
    
    // Find results by patient and time range
    let patient_results = storage.test_results()
        .find_by_patient_and_timerange("PAT001", start_date, end_date)
        .await?;
    
    // Find abnormal results
    let abnormal = storage.test_results().find_abnormal_results(Some(100)).await?;
    
    Ok(())
}
```

## Benefits of This Architecture

### 1. Separation of Concerns
- Business logic separated from data access
- Clear interfaces between layers
- Easy to test individual components

### 2. Testability
- Repository traits can be mocked
- Unit tests for business logic
- Integration tests for storage layer

### 3. Flexibility
- Easy to switch storage backends
- Support for different databases
- Pluggable storage implementations

### 4. Maintainability
- Clear, consistent patterns
- Well-defined interfaces
- Easy to extend and modify

### 5. Performance
- Optimized database schema
- Strategic indexing
- Efficient query patterns

## Future Enhancements

### 1. Additional Storage Backends
- PostgreSQL for larger deployments
- MongoDB for document storage
- Redis for caching

### 2. Advanced Features
- Full-text search capabilities
- Audit logging
- Data archiving
- Backup automation

### 3. Integration Features
- HL7 FHIR support
- REST API endpoints
- WebSocket real-time updates
- External system connectors

### 4. Analytics and Reporting
- Data aggregation
- Statistical analysis
- Custom report generation
- Dashboard metrics

## Implementation Status

- [x] Core trait definitions
- [x] SQLite storage service
- [x] Analyzer repository implementation
- [ ] Patient repository implementation
- [ ] Test result repository implementation
- [ ] Sample repository implementation
- [ ] Test order repository implementation
- [ ] Upload repository implementation
- [ ] System settings repository implementation
- [ ] Database migrations
- [ ] Unit tests
- [ ] Integration tests
- [ ] Documentation

## Next Steps

1. Complete all repository implementations
2. Add comprehensive error handling
3. Implement database migrations
4. Add unit and integration tests
5. Create usage examples and documentation
6. Performance optimization
7. Security hardening 