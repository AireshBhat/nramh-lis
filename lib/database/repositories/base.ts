import { QueryResult, SearchOptions } from '../types';
import { DatabaseClient } from '../client';

/**
 * Base repository interface for all database entities
 * Provides common CRUD operations and search functionality
 */
export interface BaseRepository<T, CreateDTO, UpdateDTO> {
  /**
   * Create a new entity
   */
  create(data: CreateDTO): Promise<T>;

  /**
   * Find entity by ID
   */
  findById(id: string): Promise<T | null>;

  /**
   * Find all entities with optional pagination
   */
  findAll(options?: SearchOptions): Promise<QueryResult<T>>;

  /**
   * Update an existing entity
   */
  update(id: string, data: UpdateDTO): Promise<T>;

  /**
   * Delete an entity by ID
   */
  delete(id: string): Promise<boolean>;

  /**
   * Count total number of entities
   */
  count(): Promise<number>;

  /**
   * Check if entity exists by ID
   */
  exists(id: string): Promise<boolean>;
}

/**
 * Abstract base repository implementation
 * Provides common functionality for all repositories
 */
export abstract class BaseRepositoryImpl<T, CreateDTO, UpdateDTO, RowType> 
  implements BaseRepository<T, CreateDTO, UpdateDTO> {
  
  protected abstract tableName: string;
  protected abstract idColumn: string;

  constructor(protected db: DatabaseClient) {}

  /**
   * Create a new entity
   */
  abstract create(data: CreateDTO): Promise<T>;

  /**
   * Find entity by ID
   */
  async findById(id: string): Promise<T | null> {
    const sql = `SELECT * FROM ${this.tableName} WHERE ${this.idColumn} = $1`;
    const result = await this.db.executeSingle(sql, [id]) as RowType | null;
    return result ? this.mapRowToEntity(result) : null;
  }

  /**
   * Find all entities with optional pagination
   */
  async findAll(options: SearchOptions = {}): Promise<QueryResult<T>> {
    const { limit = 100, offset = 0, orderBy = 'created_at', orderDirection = 'DESC' } = options;

    // Get total count
    const countSql = `SELECT COUNT(*) as total FROM ${this.tableName}`;
    const countResult = await this.db.executeSingle(countSql) as { total: number } | null;
    const total = countResult?.total || 0;

    // Get paginated data
    const sql = `
      SELECT * FROM ${this.tableName} 
      ORDER BY ${orderBy} ${orderDirection}
      LIMIT $1 OFFSET $2
    `;
    const results = await this.db.execute(sql, [limit, offset]) as RowType[];
    const data = results.map((row: RowType) => this.mapRowToEntity(row));

    return {
      data,
      total,
      limit,
      offset
    };
  }

  /**
   * Update an existing entity
   */
  abstract update(id: string, data: UpdateDTO): Promise<T>;

  /**
   * Delete an entity by ID
   */
  async delete(id: string): Promise<boolean> {
    const sql = `DELETE FROM ${this.tableName} WHERE ${this.idColumn} = $1`;
    const rowsAffected = await this.db.executeUpdate(sql, [id]);
    return rowsAffected > 0;
  }

  /**
   * Count total number of entities
   */
  async count(): Promise<number> {
    const sql = `SELECT COUNT(*) as total FROM ${this.tableName}`;
    const result = await this.db.executeSingle(sql) as { total: number } | null;
    return result?.total || 0;
  }

  /**
   * Check if entity exists by ID
   */
  async exists(id: string): Promise<boolean> {
    const sql = `SELECT 1 FROM ${this.tableName} WHERE ${this.idColumn} = $1 LIMIT 1`;
    const result = await this.db.executeSingle(sql, [id]) as { '1': number } | null;
    return !!result;
  }

  /**
   * Map database row to entity object
   */
  protected abstract mapRowToEntity(row: RowType): T;

  /**
   * Map entity object to database row
   */
  protected abstract mapEntityToRow(entity: T): Partial<RowType>;

  /**
   * Generate a new UUID
   */
  protected generateId(): string {
    return crypto.randomUUID();
  }

  /**
   * Get current timestamp in ISO format
   */
  protected getCurrentTimestamp(): string {
    return new Date().toISOString();
  }
} 