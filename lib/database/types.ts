/**
 * Database-specific error types
 */
export class DatabaseError extends Error {
  constructor(
    message: string,
    public code: string,
    public details?: any
  ) {
    super(message);
    this.name = 'DatabaseError';
  }
}

export class ValidationError extends Error {
  constructor(
    message: string,
    public field: string,
    public value: any
  ) {
    super(message);
    this.name = 'ValidationError';
  }
}

/**
 * Database row interfaces (raw database format)
 */
export interface PatientRow {
  id: string;
  last_name: string | null;
  first_name: string | null;
  middle_name: string | null;
  title: string | null;
  birth_date: string | null;
  sex: 'M' | 'F' | 'U';
  street: string | null;
  city: string | null;
  state: string | null;
  zip: string | null;
  country_code: string | null;
  telephone: string | null; // JSON string
  ordering_physician: string | null;
  attending_physician: string | null;
  referring_physician: string | null;
  height_value: number | null;
  height_unit: string | null;
  weight_value: number | null;
  weight_unit: string | null;
  created_at: string;
  updated_at: string;
}

export interface TestResultRow {
  id: string;
  test_id: string;
  sample_id: string;
  value: string;
  units: string | null;
  reference_range_lower: number | null;
  reference_range_upper: number | null;
  abnormal_flag: string | null;
  nature_of_abnormality: string | null;
  status: 'C' | 'F' | 'P';
  sequence_number: number;
  instrument: string | null;
  completed_date_time: string | null;
  analyzer_id: string | null;
  patient_id: string;
  created_at: string;
  updated_at: string;
}

/**
 * DTO interfaces for create/update operations
 */
export interface CreatePatientDTO {
  id: string
  name: {
    lastName?: string;
    firstName?: string;
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
    height?: {
      value: number;
      unit: string;
    };
    weight?: {
      value: number;
      unit: string;
    };
  };
}

export interface UpdatePatientDTO extends Partial<CreatePatientDTO> {}

export interface CreateTestResultDTO {
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
  patientId: string;
}

export interface UpdateTestResultDTO extends Partial<CreateTestResultDTO> {}

/**
 * Query result interfaces
 */
export interface QueryResult<T> {
  data: T[];
  total: number;
  limit: number;
  offset: number;
}

export interface SearchOptions {
  limit?: number;
  offset?: number;
  orderBy?: string;
  orderDirection?: 'ASC' | 'DESC';
} 