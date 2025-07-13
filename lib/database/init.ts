import { initializeDatabase, getDatabaseClient } from './client';

/**
 * Database initialization status
 */
export interface DatabaseStatus {
  initialized: boolean;
  tablesExist: boolean;
  error?: string;
}

let dbStatus: DatabaseStatus = {
  initialized: false,
  tablesExist: false,
};

/**
 * Initialize the database and verify tables exist
 */
export async function initializeDatabaseWithVerification(): Promise<DatabaseStatus> {
  try {
    // Initialize database connection
    await initializeDatabase();
    dbStatus.initialized = true;

    // Verify tables exist
    const tablesExist = await verifyTables();
    dbStatus.tablesExist = tablesExist;

    if (!tablesExist) {
      dbStatus.error = 'Required database tables do not exist. Please run migrations.';
    }

    console.log('Database initialization completed:', dbStatus);
    return dbStatus;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown database error';
    dbStatus.error = errorMessage;
    dbStatus.initialized = false;
    dbStatus.tablesExist = false;
    
    console.error('Database initialization failed:', error);
    return dbStatus;
  }
}

/**
 * Verify that required tables exist in the database
 */
async function verifyTables(): Promise<boolean> {
  try {
    const db = getDatabaseClient();
    
    // Check if patients table exists
    const patientsTableExists = await db.executeSingle(`
      SELECT name FROM sqlite_master 
      WHERE type='table' AND name='patients'
    `);
    
    // Check if test_results table exists
    const testResultsTableExists = await db.executeSingle(`
      SELECT name FROM sqlite_master 
      WHERE type='table' AND name='test_results'
    `);

    return !!(patientsTableExists && testResultsTableExists);
  } catch (error) {
    console.error('Error verifying tables:', error);
    return false;
  }
}

/**
 * Get current database status
 */
export function getDatabaseStatus(): DatabaseStatus {
  return { ...dbStatus };
}

/**
 * Check if database is ready for use
 */
export function isDatabaseReady(): boolean {
  return dbStatus.initialized && dbStatus.tablesExist && !dbStatus.error;
}

/**
 * Reset database status (useful for testing)
 */
export function resetDatabaseStatus(): void {
  dbStatus = {
    initialized: false,
    tablesExist: false,
  };
} 