// Database client and initialization
export { DatabaseClient, getDatabaseClient, initializeDatabase } from './client';
export { initializeDatabaseWithVerification, getDatabaseStatus, isDatabaseReady } from './init';

// Repository implementations
export { PatientRepository } from './repositories/patients';
export { TestResultRepository } from './repositories/test-results';

// Base repository interface
export { BaseRepositoryImpl } from './repositories/base';
export type { BaseRepository } from './repositories/base';

// Types and error classes
export { DatabaseError, ValidationError } from './types';
export type {
  PatientRow,
  TestResultRow,
  CreatePatientDTO,
  UpdatePatientDTO,
  CreateTestResultDTO,
  UpdateTestResultDTO,
  QueryResult,
  SearchOptions,
} from './types';

// React hooks
export { usePatients } from '../../hooks/use-patients';
export { useTestResults } from '../../hooks/use-test-results'; 