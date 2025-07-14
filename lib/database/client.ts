import Database from '@tauri-apps/plugin-sql';

/**
 * Database client wrapper for Tauri SQL plugin
 * Provides a clean interface for database operations
 */
export class DatabaseClient {
  private db: Database | null = null;
  private isInitialized = false;

  /**
   * Initialize the database connection
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) {
      return;
    }

    try {
      this.db = await Database.load('sqlite:nramh-lis.db');
      this.isInitialized = true;
      console.log('Database initialized successfully');
    } catch (error) {
      console.error('Failed to initialize database:', error);
      throw new Error(`Database initialization failed: ${error}`);
    }
  }

  /**
   * Execute a SELECT query and return multiple results with retry logic for database locks
   */
  async execute<T = any>(sql: string, bindValues?: any[]): Promise<T[]> {
    if (!this.db || !this.isInitialized) {
      throw new Error('Database not initialized. Call initialize() first.');
    }

    const maxRetries = 3;
    const baseDelay = 100; // 100ms base delay

    for (let attempt = 1; attempt <= maxRetries; attempt++) {
      try {
        return await this.db.select(sql, bindValues || []);
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        
        // Check if it's a database lock error
        if (errorMessage.includes('database is locked') || errorMessage.includes('SQLITE_BUSY')) {
          if (attempt < maxRetries) {
            const delay = baseDelay * Math.pow(2, attempt - 1); // Exponential backoff
            console.warn(`Database locked, retrying in ${delay}ms (attempt ${attempt}/${maxRetries})`);
            await new Promise(resolve => setTimeout(resolve, delay));
            continue;
          }
        }
        
        console.error('Database query failed:', { sql, bindValues, error, attempt });
        throw new Error(`Query execution failed: ${error}`);
      }
    }

    throw new Error('Database query failed after all retries');
  }

  /**
   * Execute a SELECT query and return a single result
   */
  async executeSingle<T = any>(sql: string, bindValues?: any[]): Promise<T | null> {
    const results = await this.execute<T>(sql, bindValues);
    return results[0] || null;
  }

  /**
   * Execute an INSERT, UPDATE, or DELETE query with retry logic for database locks
   */
  async executeUpdate(sql: string, bindValues?: any[]): Promise<number> {
    if (!this.db || !this.isInitialized) {
      throw new Error('Database not initialized. Call initialize() first.');
    }

    const maxRetries = 3;
    const baseDelay = 100; // 100ms base delay

    for (let attempt = 1; attempt <= maxRetries; attempt++) {
      try {
        const result = await this.db.execute(sql, bindValues || []);
        console.log('result', result);
        return result.rowsAffected || 0;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        
        // Check if it's a database lock error
        if (errorMessage.includes('database is locked') || errorMessage.includes('SQLITE_BUSY')) {
          if (attempt < maxRetries) {
            const delay = baseDelay * Math.pow(2, attempt - 1); // Exponential backoff
            console.warn(`Database locked, retrying in ${delay}ms (attempt ${attempt}/${maxRetries})`);
            await new Promise(resolve => setTimeout(resolve, delay));
            continue;
          }
        }
        
        console.error('Database update failed:', { sql, bindValues, error, attempt });
        throw new Error(`Update execution failed: ${error}`);
      }
    }

    throw new Error('Database update failed after all retries');
  }

  /**
   * Execute a transaction with multiple operations and retry logic for database locks
   */
  async transaction<T>(operations: () => Promise<T>): Promise<T> {
    if (!this.db || !this.isInitialized) {
      throw new Error('Database not initialized. Call initialize() first.');
    }

    const maxRetries = 3;
    const baseDelay = 100; // 100ms base delay

    for (let attempt = 1; attempt <= maxRetries; attempt++) {
      try {
        await this.db.execute('BEGIN TRANSACTION');
        const result = await operations();
        await this.db.execute('COMMIT');
        return result;
      } catch (error) {
        // Always try to rollback on error
        if (this.db) {
          try {
            await this.db.execute('ROLLBACK');
          } catch (rollbackError) {
            console.error('Error during rollback:', rollbackError);
          }
        }

        const errorMessage = error instanceof Error ? error.message : String(error);
        
        // Check if it's a database lock error
        if (errorMessage.includes('database is locked') || errorMessage.includes('SQLITE_BUSY')) {
          if (attempt < maxRetries) {
            const delay = baseDelay * Math.pow(2, attempt - 1); // Exponential backoff
            console.warn(`Database locked during transaction, retrying in ${delay}ms (attempt ${attempt}/${maxRetries})`);
            await new Promise(resolve => setTimeout(resolve, delay));
            continue;
          }
        }
        
        // For non-lock errors or after all retries, throw the error
        throw error;
      }
    }

    throw new Error('Transaction failed after all retries');
  }

  /**
   * Check if database is initialized
   */
  get initialized(): boolean {
    return this.isInitialized;
  }

  /**
   * Close the database connection
   */
  async close(): Promise<void> {
    if (this.db) {
      await this.db.close();
      this.db = null;
      this.isInitialized = false;
    }
  }
}

// Singleton instance
let dbClient: DatabaseClient | null = null;

/**
 * Get the global database client instance
 */
export function getDatabaseClient(): DatabaseClient {
  if (!dbClient) {
    dbClient = new DatabaseClient();
  }
  return dbClient;
}

/**
 * Initialize the global database client
 */
export async function initializeDatabase(): Promise<void> {
  const client = getDatabaseClient();
  await client.initialize();
} 