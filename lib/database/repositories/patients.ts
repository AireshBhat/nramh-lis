import { Patient } from '@/lib/types';
import { BaseRepositoryImpl } from './base';
import { DatabaseClient } from '../client';
import { 
  PatientRow, 
  CreatePatientDTO, 
  UpdatePatientDTO, 
  ValidationError 
} from '../types';

/**
 * Patient repository implementation
 * Provides CRUD operations and specialized search methods for patients
 */
export class PatientRepository extends BaseRepositoryImpl<Patient, CreatePatientDTO, UpdatePatientDTO, PatientRow> {
  protected tableName = 'patients';
  protected idColumn = 'id';

  constructor(db: DatabaseClient) {
    super(db);
  }

  /**
   * Create a new patient
   */
  async create(data: CreatePatientDTO): Promise<Patient> {
    // Validate required fields
    this.validateCreateData(data);

    const id = data.id;
    const now = this.getCurrentTimestamp();

    const sql = `
      INSERT INTO patients (
        id, last_name, first_name, middle_name, title, birth_date, sex,
        street, city, state, zip, country_code, telephone,
        ordering_physician, attending_physician, referring_physician,
        height_value, height_unit, weight_value, weight_unit,
        created_at, updated_at
      ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
    `;

    const bindValues = [
      id,
      data.name.lastName || null,
      data.name.firstName || null,
      data.name.middleName || null,
      data.name.title || null,
      data.birthDate?.toISOString() || null,
      this.mapSexToDatabase(data.sex),
      data.address?.street || null,
      data.address?.city || null,
      data.address?.state || null,
      data.address?.zip || null,
      data.address?.countryCode || null,
      JSON.stringify(data.telephone),
      data.physicians?.ordering && data.physicians?.ordering !== "0" ? data.physicians.ordering : null,
      data.physicians?.attending && data.physicians?.attending !== "0" ? data.physicians.attending : null,
      data.physicians?.referring && data.physicians?.referring !== "0" ? data.physicians.referring : null,
      data.physicalAttributes?.height?.value || null,
      data.physicalAttributes?.height?.unit || null,
      data.physicalAttributes?.weight?.value || null,
      data.physicalAttributes?.weight?.unit || null,
      now,
      now
    ];

    console.log('SQL:', sql);
    console.log('Bind values count:', bindValues.length);
    console.log('Bind values:', bindValues);

    await this.db.executeUpdate(sql, bindValues);
    return this.findById(id) as Promise<Patient>;
  }

  /**
   * Update an existing patient
   */
  async update(id: string, data: UpdatePatientDTO): Promise<Patient> {
    // Check if patient exists
    const existing = await this.findById(id);
    if (!existing) {
      throw new ValidationError('Patient not found', 'id', id);
    }

    const now = this.getCurrentTimestamp();
    const updates: string[] = [];
    const bindValues: any[] = [];

    // Build dynamic update query
    if (data.name?.lastName !== undefined) {
      updates.push('last_name = ?');
      bindValues.push(data.name.lastName);
    }
    if (data.name?.firstName !== undefined) {
      updates.push('first_name = ?');
      bindValues.push(data.name.firstName);
    }
    if (data.name?.middleName !== undefined) {
      updates.push('middle_name = ?');
      bindValues.push(data.name.middleName);
    }
    if (data.name?.title !== undefined) {
      updates.push('title = ?');
      bindValues.push(data.name.title);
    }
    if (data.birthDate !== undefined) {
      updates.push('birth_date = ?');
      bindValues.push(data.birthDate?.toISOString() || null);
    }
    if (data.sex !== undefined) {
      updates.push('sex = ?');
      bindValues.push(this.mapSexToDatabase(data.sex));
    }
    if (data.address?.street !== undefined) {
      updates.push('street = ?');
      bindValues.push(data.address.street);
    }
    if (data.address?.city !== undefined) {
      updates.push('city = ?');
      bindValues.push(data.address.city);
    }
    if (data.address?.state !== undefined) {
      updates.push('state = ?');
      bindValues.push(data.address.state);
    }
    if (data.address?.zip !== undefined) {
      updates.push('zip = ?');
      bindValues.push(data.address.zip);
    }
    if (data.address?.countryCode !== undefined) {
      updates.push('country_code = ?');
      bindValues.push(data.address.countryCode);
    }
    if (data.telephone !== undefined) {
      updates.push('telephone = ?');
      bindValues.push(JSON.stringify(data.telephone));
    }
    if (data.physicians?.ordering !== undefined) {
      updates.push('ordering_physician = ?');
      bindValues.push(data.physicians.ordering);
    }
    if (data.physicians?.attending !== undefined) {
      updates.push('attending_physician = ?');
      bindValues.push(data.physicians.attending);
    }
    if (data.physicians?.referring !== undefined) {
      updates.push('referring_physician = ?');
      bindValues.push(data.physicians.referring);
    }
    if (data.physicalAttributes?.height?.value !== undefined) {
      updates.push('height_value = ?');
      bindValues.push(data.physicalAttributes.height.value);
    }
    if (data.physicalAttributes?.height?.unit !== undefined) {
      updates.push('height_unit = ?');
      bindValues.push(data.physicalAttributes.height.unit);
    }
    if (data.physicalAttributes?.weight?.value !== undefined) {
      updates.push('weight_value = ?');
      bindValues.push(data.physicalAttributes.weight.value);
    }
    if (data.physicalAttributes?.weight?.unit !== undefined) {
      updates.push('weight_unit = ?');
      bindValues.push(data.physicalAttributes.weight.unit);
    }

    // Always update the updated_at timestamp
    updates.push('updated_at = ?');
    bindValues.push(now);

    if (updates.length === 1) {
      // Only updated_at was added, no actual changes
      return existing;
    }

    const sql = `UPDATE ${this.tableName} SET ${updates.join(', ')} WHERE ${this.idColumn} = ?`;
    bindValues.push(id);

    await this.db.executeUpdate(sql, bindValues);
    return this.findById(id) as Promise<Patient>;
  }

  /**
   * Search patients by name
   */
  async searchByName(lastName?: string, firstName?: string, limit: number = 100): Promise<Patient[]> {
    let sql = `SELECT * FROM ${this.tableName} WHERE 1=1`;
    const params: any[] = [];

    if (lastName) {
      sql += ' AND last_name LIKE $' + (params.length + 1);
      params.push(`%${lastName}%`);
    }

    if (firstName) {
      sql += ' AND first_name LIKE $' + (params.length + 1);
      params.push(`%${firstName}%`);
    }

    sql += ' ORDER BY last_name, first_name LIMIT $' + (params.length + 1);
    params.push(limit);

    const results = await this.db.execute(sql, params) as PatientRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find patients by birth date range
   */
  async findByBirthDateRange(startDate: Date, endDate: Date, limit: number = 100): Promise<Patient[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE birth_date BETWEEN $1 AND $2
      ORDER BY birth_date DESC
      LIMIT $3
    `;

    const results = await this.db.execute(sql, [
      startDate.toISOString(),
      endDate.toISOString(),
      limit
    ]) as PatientRow[];

    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Find recent patients (created within specified days)
   */
  async findRecentPatients(days: number = 30, limit: number = 100): Promise<Patient[]> {
    const sql = `
      SELECT * FROM ${this.tableName} 
      WHERE created_at >= datetime('now', '-${days} days')
      ORDER BY created_at DESC
      LIMIT $1
    `;

    const results = await this.db.execute(sql, [limit]) as PatientRow[];
    return results.map(row => this.mapRowToEntity(row));
  }

  /**
   * Map database row to Patient entity
   */
  protected mapRowToEntity(row: PatientRow): Patient {
    return {
      id: row.id,
      name: {
        lastName: row.last_name || undefined,
        firstName: row.first_name || undefined,
        middleName: row.middle_name || undefined,
        title: row.title || undefined,
      },
      birthDate: row.birth_date ? new Date(row.birth_date) : undefined,
      sex: this.mapSexFromDatabase(row.sex),
      address: row.street || row.city || row.state || row.zip || row.country_code ? {
        street: row.street || undefined,
        city: row.city || undefined,
        state: row.state || undefined,
        zip: row.zip || undefined,
        countryCode: row.country_code || undefined,
      } : undefined,
      telephone: row.telephone ? JSON.parse(row.telephone) : [],
      physicians: row.ordering_physician || row.attending_physician || row.referring_physician ? {
        ordering: row.ordering_physician || undefined,
        attending: row.attending_physician || undefined,
        referring: row.referring_physician || undefined,
      } : undefined,
      physicalAttributes: row.height_value || row.weight_value ? {
        height: row.height_value && row.height_unit ? {
          value: row.height_value,
          unit: row.height_unit,
        } : undefined,
        weight: row.weight_value && row.weight_unit ? {
          value: row.weight_value,
          unit: row.weight_unit,
        } : undefined,
      } : undefined,
      createdAt: new Date(row.created_at),
      updatedAt: new Date(row.updated_at),
    };
  }

  /**
   * Map Patient entity to database row
   */
  protected mapEntityToRow(entity: Patient): Partial<PatientRow> {
    return {
      id: entity.id,
      last_name: entity.name.lastName || null,
      first_name: entity.name.firstName || null,
      middle_name: entity.name.middleName || null,
      title: entity.name.title || null,
      birth_date: entity.birthDate?.toISOString() || null,
      sex: this.mapSexToDatabase(entity.sex),
      street: entity.address?.street || null,
      city: entity.address?.city || null,
      state: entity.address?.state || null,
      zip: entity.address?.zip || null,
      country_code: entity.address?.countryCode || null,
      telephone: JSON.stringify(entity.telephone),
      ordering_physician: entity.physicians?.ordering || null,
      attending_physician: entity.physicians?.attending || null,
      referring_physician: entity.physicians?.referring || null,
      height_value: entity.physicalAttributes?.height?.value || null,
      height_unit: entity.physicalAttributes?.height?.unit || null,
      weight_value: entity.physicalAttributes?.weight?.value || null,
      weight_unit: entity.physicalAttributes?.weight?.unit || null,
      created_at: entity.createdAt.toISOString(),
      updated_at: entity.updatedAt.toISOString(),
    };
  }

  /**
   * Map frontend sex enum to database format
   */
  private mapSexToDatabase(sex: 'Male' | 'Female' | 'Other'): 'M' | 'F' | 'U' {
    switch (sex) {
      case 'Male': return 'M';
      case 'Female': return 'F';
      case 'Other': return 'U';
      default: return 'U';
    }
  }

  /**
   * Map database sex format to frontend enum
   */
  private mapSexFromDatabase(sex: 'M' | 'F' | 'U'): 'Male' | 'Female' | 'Other' {
    switch (sex) {
      case 'M': return 'Male';
      case 'F': return 'Female';
      case 'U': return 'Other';
      default: return 'Other';
    }
  }

  /**
   * Validate create data
   */
  private validateCreateData(data: CreatePatientDTO): void {
    // if (!data.telephone || data.telephone.length === 0) {
    //   throw new ValidationError('At least one telephone number is required', 'telephone', data.telephone);
    // }

    // if (data.physicalAttributes?.height?.value && !data.physicalAttributes.height.unit) {
    //   throw new ValidationError('Height unit is required when height value is provided', 'height.unit', data.physicalAttributes.height.unit);
    // }

    // if (data.physicalAttributes?.weight?.value && !data.physicalAttributes.weight.unit) {
    //   throw new ValidationError('Weight unit is required when weight value is provided', 'weight.unit', data.physicalAttributes.weight.unit);
    // }
  }
} 