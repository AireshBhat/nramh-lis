# Database Implementation for NRAMH LIS 2

This directory contains the complete database implementation for the Laboratory Information System using Tauri-SQL plugin with industry-standard patterns.

## Architecture Overview

The database implementation follows the **Repository Pattern** with the following layers:

1. **Database Client** - Low-level SQL operations wrapper
2. **Repository Layer** - Business logic and data access abstraction
3. **React Hooks** - UI integration layer
4. **Type Definitions** - TypeScript interfaces and error handling

## File Structure

```
lib/database/
├── client.ts              # Database client wrapper
├── init.ts                # Database initialization
├── types.ts               # Type definitions and DTOs
├── index.ts               # Main exports
├── repositories/
│   ├── base.ts            # Base repository interface
│   ├── patients.ts        # Patient repository
│   └── test-results.ts    # Test results repository
└── README.md              # This file
```

## Quick Start

### 1. Initialize Database

```typescript
import { initializeDatabaseWithVerification } from '@/lib/database';

// In your app initialization
const dbStatus = await initializeDatabaseWithVerification();
if (!dbStatus.initialized) {
  console.error('Database initialization failed:', dbStatus.error);
}
```

### 2. Use React Hooks

```typescript
import { usePatients, useTestResults } from '@/lib/database';

function MyComponent() {
  const {
    patients,
    loading,
    error,
    createPatient,
    searchPatients
  } = usePatients({ autoLoad: true });

  const {
    testResults,
    statistics,
    findAbnormalResults
  } = useTestResults({ autoLoad: true });

  // Your component logic here
}
```

## Core Components

### Database Client

The `DatabaseClient` provides a clean interface for SQL operations:

```typescript
import { getDatabaseClient } from '@/lib/database';

const db = getDatabaseClient();

// Execute queries
const results = await db.execute('SELECT * FROM patients WHERE id = $1', [patientId]);
const single = await db.executeSingle('SELECT * FROM patients WHERE id = $1', [patientId]);

// Execute updates
const rowsAffected = await db.executeUpdate(
  'UPDATE patients SET name = $1 WHERE id = $2', 
  [newName, patientId]
);

// Transactions
await db.transaction(async () => {
  await db.executeUpdate('INSERT INTO patients (...) VALUES (...)');
  await db.executeUpdate('INSERT INTO test_results (...) VALUES (...)');
});
```

### Repository Pattern

Each entity has a dedicated repository with CRUD operations and specialized queries:

```typescript
import { PatientRepository, TestResultRepository } from '@/lib/database';

const db = getDatabaseClient();
const patientRepo = new PatientRepository(db);
const testResultRepo = new TestResultRepository(db);

// Patient operations
const patient = await patientRepo.create(patientData);
const patients = await patientRepo.searchByName('Smith', 'John');
const recentPatients = await patientRepo.findRecentPatients(30);

// Test result operations
const results = await testResultRepo.findBySampleId('SAMPLE001');
const abnormalResults = await testResultRepo.findAbnormalResults(50);
const stats = await testResultRepo.getStatistics();
```

### React Hooks

The hooks provide a React-friendly interface with state management:

```typescript
// Patient hook
const {
  patients,           // Current patients list
  loading,           // Loading state
  error,             // Error state
  createPatient,     // Create new patient
  updatePatient,     // Update existing patient
  deletePatient,     // Delete patient
  searchPatients,    // Search by name
  refresh           // Refresh data
} = usePatients({ autoLoad: true, limit: 100 });

// Test results hook
const {
  testResults,       // Current test results list
  statistics,        // Test result statistics
  findBySampleId,    // Find by sample
  findAbnormalResults, // Find abnormal results
  batchInsert       // Batch insert multiple results
} = useTestResults({ autoLoad: true, limit: 100 });
```

## Data Types

### Patient DTOs

```typescript
interface CreatePatientDTO {
  name: {
    firstName?: string;
    lastName?: string;
    middleName?: string;
    title?: string;
  };
  birthDate?: Date;
  sex: 'Male' | 'Female' | 'Other';
  address?: {
    street?: string;
    city?: string;
    state?: string;
    zip?: string;
    countryCode?: string;
  };
  telephone: string[];
  physicians?: {
    ordering?: string;
    attending?: string;
    referring?: string;
  };
  physicalAttributes?: {
    height?: { value: number; unit: string; };
    weight?: { value: number; unit: string; };
  };
}
```

### Test Result DTOs

```typescript
interface CreateTestResultDTO {
  testId: string;
  sampleId: string;
  value: string;
  units?: string;
  referenceRange?: {
    lowerLimit?: number;
    upperLimit?: number;
  };
  flags?: {
    abnormalFlag?: string;
    natureOfAbnormality?: string;
  };
  status: 'Correction' | 'Final' | 'Preliminary';
  completedDateTime?: Date;
  metadata: {
    sequenceNumber: number;
    instrument?: string;
  };
  analyzerId?: string;
}
```

## Error Handling

The implementation includes comprehensive error handling:

```typescript
import { DatabaseError, ValidationError } from '@/lib/database';

try {
  const patient = await createPatient(patientData);
} catch (error) {
  if (error instanceof ValidationError) {
    console.error(`Validation error in ${error.field}:`, error.message);
  } else if (error instanceof DatabaseError) {
    console.error('Database error:', error.message);
  } else {
    console.error('Unknown error:', error);
  }
}
```

## Advanced Features

### Batch Operations

```typescript
// Batch insert multiple test results
const testResults = [result1, result2, result3];
const ids = await batchInsert(testResults);
```

### Statistics

```typescript
// Get test result statistics
const stats = await testResultRepo.getStatistics();
console.log(`Total: ${stats.total}, Abnormal: ${stats.abnormal}`);
```

### Pagination

```typescript
// Paginated results
const result = await patientRepo.findAll({
  limit: 20,
  offset: 40,
  orderBy: 'created_at',
  orderDirection: 'DESC'
});
```

## Database Schema

The implementation works with the following tables:

### Patients Table
```sql
CREATE TABLE patients (
  id TEXT PRIMARY KEY NOT NULL,
  last_name TEXT,
  first_name TEXT,
  middle_name TEXT,
  title TEXT,
  birth_date TEXT,
  sex TEXT NOT NULL CHECK (sex IN ('M', 'F', 'U')),
  street TEXT,
  city TEXT,
  state TEXT,
  zip TEXT,
  country_code TEXT,
  telephone TEXT, -- JSON array
  ordering_physician TEXT,
  attending_physician TEXT,
  referring_physician TEXT,
  height_value REAL,
  height_unit TEXT,
  weight_value REAL,
  weight_unit TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

### Test Results Table
```sql
CREATE TABLE test_results (
  id TEXT PRIMARY KEY NOT NULL,
  test_id TEXT NOT NULL,
  sample_id TEXT NOT NULL,
  value TEXT NOT NULL,
  units TEXT,
  reference_range_lower REAL,
  reference_range_upper REAL,
  abnormal_flag TEXT,
  nature_of_abnormality TEXT,
  status TEXT NOT NULL CHECK (status IN ('C', 'F', 'P')),
  sequence_number INTEGER NOT NULL,
  instrument TEXT,
  completed_date_time TEXT,
  analyzer_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

## Performance Considerations

1. **Indexing**: The database includes indexes on frequently queried columns
2. **Pagination**: All list operations support pagination to handle large datasets
3. **Batch Operations**: Use batch insert for multiple records
4. **Connection Pooling**: The client manages database connections efficiently

## Testing

To test the database implementation:

```typescript
import { DatabaseExample } from '@/components/database-example';

// Add to your page
<DatabaseExample />
```

This component demonstrates all major database operations and can be used for testing and development.

## Migration

The database uses Tauri's built-in migration system. Migrations are defined in `src-tauri/src/migrations.rs` and are automatically applied when the app starts.

## Best Practices

1. **Always use transactions** for operations that modify multiple tables
2. **Handle errors gracefully** with proper error boundaries
3. **Use pagination** for large datasets
4. **Validate data** before inserting into the database
5. **Use the React hooks** for UI components instead of direct repository access
6. **Monitor performance** with the statistics features

## Troubleshooting

### Common Issues

1. **Database not initialized**: Ensure `initializeDatabaseWithVerification()` is called before using any database operations
2. **Tables don't exist**: Check that migrations are running correctly
3. **Type errors**: Ensure you're using the correct DTO types for create/update operations
4. **Performance issues**: Use pagination and limit the number of records loaded

### Debug Mode

Enable debug logging to see SQL queries:

```typescript
// In your database client
console.log('Executing SQL:', sql, bindValues);
```

## Future Enhancements

1. **Caching**: Implement Redis or in-memory caching for frequently accessed data
2. **Full-text Search**: Add full-text search capabilities for patient names
3. **Audit Logging**: Track all database changes for compliance
4. **Backup/Restore**: Automated database backup and restore functionality
5. **Multi-tenancy**: Support for multiple laboratory locations 