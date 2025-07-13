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
   * Execute a SELECT query and return multiple results
   */
  async execute<T = any>(sql: string, bindValues?: any[]): Promise<T[]> {
    if (!this.db || !this.isInitialized) {
      throw new Error('Database not initialized. Call initialize() first.');
    }

    try {
      return await this.db.select(sql, bindValues || []);
    } catch (error) {
      console.error('Database query failed:', { sql, bindValues, error });
      throw new Error(`Query execution failed: ${error}`);
    }
  }

  /**
   * Execute a SELECT query and return a single result
   */
  async executeSingle<T = any>(sql: string, bindValues?: any[]): Promise<T | null> {
    const results = await this.execute<T>(sql, bindValues);
    return results[0] || null;
  }

  /**
   * Execute an INSERT, UPDATE, or DELETE query
   */
  async executeUpdate(sql: string, bindValues?: any[]): Promise<number> {
    if (!this.db || !this.isInitialized) {
      throw new Error('Database not initialized. Call initialize() first.');
    }

    try {
      const result = await this.db.execute(sql, bindValues || []);
      return result.rowsAffected || 0;
    } catch (error) {
      console.error('Database update failed:', { sql, bindValues, error });
      throw new Error(`Update execution failed: ${error}`);
    }
  }

  /**
   * Execute a transaction with multiple operations
   */
  async transaction<T>(operations: () => Promise<T>): Promise<T> {
    if (!this.db || !this.isInitialized) {
      throw new Error('Database not initialized. Call initialize() first.');
    }

    try {
      await this.db.execute('BEGIN TRANSACTION');
      const result = await operations();
      await this.db.execute('COMMIT');
      return result;
    } catch (error) {
      if (this.db) {
        await this.db.execute('ROLLBACK');
      }
      throw error;
    }
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