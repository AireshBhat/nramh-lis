import { TestResult } from '@/lib/types';
import { BaseRepositoryImpl } from './base';
import { DatabaseClient } from '../client';
import { 
  TestResultRow, 
  CreateTestResultDTO, 
  UpdateTestResultDTO, 
  ValidationError 
} from '../types';

/**
 * Test Result repository implementation
 * Provides CRUD operations and specialized querying for laboratory test results
 */
export class TestResultRepository extends BaseRepositoryImpl<TestResult, CreateTestResultDTO, UpdateTestResultDTO, TestResultRow> {
  protected tableName = 'test_results';
  protected idColumn = 'id';

  constructor(db: DatabaseClient) {
    super(db);
  }

  /**
   * Create a new test result
   */
  async create(data: CreateTestResultDTO): Promise<TestResult> {
    // Validate required fields
    this.validateCreateData(data);

    const id = this.generateId();
    const now = this.getCurrentTimestamp();

    const sql = `
      INSERT INTO test_results (
        id, test_id, sample_id, value, units, reference_range_lower,
        reference_range_upper, abnormal_flag, nature_of_abnormality,
        status, sequence_number, instrument, completed_date_time,
        analyzer_id, patient_id, created_at, updated_at
      ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
    `;

    const bindValues = [
      id,
      data.testId,
      data.sampleId,
      data.value,
      data.units || null,
      data.referenceRange?.lowerLimit || null,
      data.referenceRange?.upperLimit || null,
      data.flags?.abnormalFlag || null,
      data.flags?.natureOfAbnormality || null,
      this.mapStatusToDatabase(data.status),
      data.metadata.sequenceNumber,
      data.metadata.instrument || null,
      data.completedDateTime?.toISOString() || null,
      data.analyzerId || null,
      data.patientId,
      now,
      now
    ];

    await this.db.executeUpdate(sql, bindValues);
    return this.findById(id) as Promise<TestResult>;
  }

  /**
   * Update an existing test result
   */
  async update(id: string, data: UpdateTestResultDTO): Promise<TestResult> {
    // Check if test result exists
    const existing = await this.findById(id);
    if (!existing) {
      throw new ValidationError('Test result not found', 'id', id);
    }

    const now = this.getCurrentTimestamp();
    const updates: string[] = [];
    const bindValues: any[] = [];

    // Build dynamic update query with sequential placeholders
    let placeholderIndex = 1;
    
    if (data.testId !== undefined) {
      updates.push(`test_id = $${placeholderIndex++}`);
      bindValues.push(data.testId);
    }
    if (data.sampleId !== undefined) {
      updates.push(`sample_id = $${placeholderIndex++}`);
      bindValues.push(data.sampleId);
    }
    if (data.value !== undefined) {
      updates.push(`value = $${placeholderIndex++}`);
      bindValues.push(data.value);
    }
    if (data.units !== undefined) {
      updates.push(`units = $${placeholderIndex++}`);
      bindValues.push(data.units);
    }
    if (data.referenceRange?.lowerLimit !== undefined) {
      updates.push(`reference_range_lower = $${placeholderIndex++}`);
      bindValues.push(data.referenceRange.lowerLimit);
    }
    if (data.referenceRange?.upperLimit !== undefined) {
      updates.push(`reference_range_upper = $${placeholderIndex++}`);
      bindValues.push(data.referenceRange.upperLimit);
    }
    if (data.flags?.abnormalFlag !== undefined) {
      updates.push(`abnormal_flag = $${placeholderIndex++}`);
      bindValues.push(data.flags.abnormalFlag);
    }
    if (data.flags?.natureOfAbnormality !== undefined) {
      updates.push(`nature_of_abnormality = $${placeholderIndex++}`);
      bindValues.push(data.flags.natureOfAbnormality);
    }
    if (data.status !== undefined) {
      updates.push(`status = $${placeholderIndex++}`);
      bindValues.push(this.mapStatusToDatabase(data.status));
    }
    if (data.metadata?.sequenceNumber !== undefined) {
      updates.push(`sequence_number = $${placeholderIndex++}`);
      bindValues.push(data.metadata.sequenceNumber);
    }
    if (data.metadata?.instrument !== undefined) {
      updates.push(`instrument = $${placeholderIndex++}`);
      bindValues.push(data.metadata.instrument);
    }
    if (data.completedDateTime !== undefined) {
      updates.push(`completed_date_time = $${placeholderIndex++}`);
      bindValues.push(data.completedDateTime?.toISOString() || null);
    }
    if (data.analyzerId !== undefined) {
      updates.push(`analyzer_id = $${placeholderIndex++}`);
      bindValues.push(data.analyzerId);
    }
    if (data.patientId !== undefined) {
      updates.push(`patient_id = $${placeholderIndex++}`);
      bindValues.push(data.patientId);
    }

    // Always update the updated_at timestamp
    updates.push(`updated_at = $${placeholderIndex++}`);
    bindValues.push(now);

    if (updates.length === 1) {
      // Only updated_at was added, no actual changes
      return existing;
    }

    const sql = `UPDATE ${this.tableName} SET ${updates.join(', ')} WHERE ${this.idColumn} = $${placeholderIndex}`;
    bindValues.push(id);

    await this.db.executeUpdate(sql, bindValues);
    return this.findById(id) as Promise<TestResult>;
  }

  /**
   * Find test results by sample ID
   */
  async findBySampleId(sampleId: string): Promise<TestResult[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE sample_id = $1 
      ORDER BY sequence_number ASC, completed_date_time DESC
    `;

    const results = await this.db.execute(sql, [sampleId]) as TestResultRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find test results by analyzer ID
   */
  async findByAnalyzerId(analyzerId: string, limit: number = 100): Promise<TestResult[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE analyzer_id = $1 
      ORDER BY completed_date_time DESC
      LIMIT $2
    `;

    const results = await this.db.execute(sql, [analyzerId, limit]) as TestResultRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find test results by date range
   */
  async findByDateRange(startDate: Date, endDate: Date, limit: number = 100): Promise<TestResult[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE completed_date_time BETWEEN $1 AND $2
      ORDER BY completed_date_time DESC
      LIMIT $3
    `;

    const results = await this.db.execute(sql, [
      startDate.toISOString(),
      endDate.toISOString(),
      limit
    ]) as TestResultRow[];

    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find abnormal test results
   */
  async findAbnormalResults(limit: number = 50): Promise<TestResult[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE abnormal_flag IS NOT NULL 
      ORDER BY completed_date_time DESC 
      LIMIT $1
    `;

    const results = await this.db.execute(sql, [limit]) as TestResultRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find test results by status
   */
  async findByStatus(status: 'Correction' | 'Final' | 'Preliminary', limit: number = 100): Promise<TestResult[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE status = $1 
      ORDER BY completed_date_time DESC
      LIMIT $2
    `;

    const results = await this.db.execute(sql, [this.mapStatusToDatabase(status), limit]) as TestResultRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find recent test results (completed within specified hours)
   */
  async findRecentResults(hours: number = 24, limit: number = 100): Promise<TestResult[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE completed_date_time >= datetime('now', '-${hours} hours')
      ORDER BY completed_date_time DESC
      LIMIT $1
    `;

    const results = await this.db.execute(sql, [limit]) as TestResultRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Batch insert multiple test results
   * Note: This method does not use its own transaction to avoid nested transaction issues.
   * It should be called within an existing transaction context.
   */
  async batchInsert(results: CreateTestResultDTO[]): Promise<string[]> {
    if (results.length === 0) {
      return [];
    }

    console.log(`Starting batch insert of ${results.length} test results`);
    const now = this.getCurrentTimestamp();
    const ids: string[] = [];
    
    for (let i = 0; i < results.length; i++) {
      const data = results[i];
      console.log(`Processing test result ${i + 1}/${results.length}: ${data.testId}`);
      
      this.validateCreateData(data);
      const id = this.generateId();

      const sql = `
        INSERT INTO test_results (
          id, test_id, sample_id, value, units, reference_range_lower,
          reference_range_upper, abnormal_flag, nature_of_abnormality,
          status, sequence_number, instrument, completed_date_time,
          analyzer_id, patient_id, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
      `;

      const bindValues = [
        id,
        data.testId,
        data.sampleId,
        data.value,
        data.units || null,
        data.referenceRange?.lowerLimit || null,
        data.referenceRange?.upperLimit || null,
        data.flags?.abnormalFlag || null,
        data.flags?.natureOfAbnormality || null,
        this.mapStatusToDatabase(data.status),
        data.metadata.sequenceNumber,
        data.metadata.instrument || null,
        data.completedDateTime?.toISOString() || null,
        data.analyzerId || null,
        data.patientId,
        now,
        now
      ];

      try {
        await this.db.executeUpdate(sql, bindValues);
        ids.push(id);
        console.log(`Successfully inserted test result ${i + 1}/${results.length}: ${id}`);
      } catch (error) {
        console.error(`Failed to insert test result ${i + 1}/${results.length}:`, error);
        throw error;
      }
    }

    console.log(`Completed batch insert of ${ids.length} test results`);
    return ids;
  }

  /**
   * Get test result statistics
   */
  async getStatistics(): Promise<{
    total: number;
    final: number;
    preliminary: number;
    correction: number;
    abnormal: number;
    today: number;
  }> {
    const sql = `
      SELECT 
        COUNT(*) as total,
        SUM(CASE WHEN status = 'F' THEN 1 ELSE 0 END) as final,
        SUM(CASE WHEN status = 'P' THEN 1 ELSE 0 END) as preliminary,
        SUM(CASE WHEN status = 'C' THEN 1 ELSE 0 END) as correction,
        SUM(CASE WHEN abnormal_flag IS NOT NULL THEN 1 ELSE 0 END) as abnormal,
        SUM(CASE WHEN completed_date_time >= date('now') THEN 1 ELSE 0 END) as today
      FROM ${this.tableName}
    `;

    const result = await this.db.executeSingle(sql) as {
      total: number;
      final: number;
      preliminary: number;
      correction: number;
      abnormal: number;
      today: number;
    } | null;

    return result || {
      total: 0,
      final: 0,
      preliminary: 0,
      correction: 0,
      abnormal: 0,
      today: 0
    };
  }

  /**
   * Map database row to TestResult entity
   */
  protected mapRowToEntity(row: TestResultRow): TestResult {
    return {
      id: row.id,
      testId: row.test_id,
      sampleId: row.sample_id,
      value: row.value,
      units: row.units || undefined,
      referenceRange: (row.reference_range_lower !== null || row.reference_range_upper !== null)
        ? {
            lowerLimit: row.reference_range_lower || undefined,
            upperLimit: row.reference_range_upper || undefined,
          }
        : undefined,
      flags: (row.abnormal_flag || row.nature_of_abnormality)
        ? {
            abnormalFlag: row.abnormal_flag || undefined,
            natureOfAbnormality: row.nature_of_abnormality || undefined,
          }
        : undefined,
      status: this.mapStatusFromDatabase(row.status),
      completedDateTime: row.completed_date_time ? new Date(row.completed_date_time) : undefined,
      metadata: {
        sequenceNumber: row.sequence_number,
        instrument: row.instrument || undefined,
      },
      analyzerId: row.analyzer_id || undefined,
      patientId: row.patient_id,
      createdAt: new Date(row.created_at),
      updatedAt: new Date(row.updated_at),
    };
  }

  /**
   * Map TestResult entity to database row
   */
  protected mapEntityToRow(entity: TestResult): Partial<TestResultRow> {
    return {
      id: entity.id,
      test_id: entity.testId,
      sample_id: entity.sampleId,
      value: entity.value,
      units: entity.units || null,
      reference_range_lower: entity.referenceRange?.lowerLimit ?? null,
      reference_range_upper: entity.referenceRange?.upperLimit ?? null,
      abnormal_flag: entity.flags?.abnormalFlag ?? null,
      nature_of_abnormality: entity.flags?.natureOfAbnormality ?? null,
      status: this.mapStatusToDatabase(entity.status),
      sequence_number: entity.metadata.sequenceNumber,
      instrument: entity.metadata.instrument || null,
      completed_date_time: entity.completedDateTime?.toISOString() || null,
      analyzer_id: entity.analyzerId || null,
      patient_id: entity.patientId, // <-- added
      created_at: entity.createdAt.toISOString(),
      updated_at: entity.updatedAt.toISOString(),
    };
  }

  /**
   * Map frontend status enum to database format
   */
  private mapStatusToDatabase(status: 'Correction' | 'Final' | 'Preliminary'): 'C' | 'F' | 'P' {
    switch (status) {
      case 'Correction': return 'C';
      case 'Final': return 'F';
      case 'Preliminary': return 'P';
      default: return 'P';
    }
  }

  /**
   * Map database status format to frontend enum
   */
  private mapStatusFromDatabase(status: 'C' | 'F' | 'P'): 'Correction' | 'Final' | 'Preliminary' {
    switch (status) {
      case 'C': return 'Correction';
      case 'F': return 'Final';
      case 'P': return 'Preliminary';
      default: return 'Preliminary';
    }
  }

  /**
   * Validate create data
   */
  private validateCreateData(data: CreateTestResultDTO): void {
    if (!data.testId || data.testId.trim() === '') {
      throw new ValidationError('Test ID is required', 'testId', data.testId);
    }

    if (!data.sampleId || data.sampleId.trim() === '') {
      throw new ValidationError('Sample ID is required', 'sampleId', data.sampleId);
    }

    if (!data.value || data.value.trim() === '') {
      throw new ValidationError('Test value is required', 'value', data.value);
    }

    if (data.referenceRange?.lowerLimit && data.referenceRange?.upperLimit) {
      if (data.referenceRange.lowerLimit >= data.referenceRange.upperLimit) {
        throw new ValidationError(
          'Lower limit must be less than upper limit', 
          'referenceRange', 
          data.referenceRange
        );
      }
    }
  }
} 