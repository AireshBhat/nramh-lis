export interface ConnectionType {
  type: 'Serial' | 'TcpIp';
}

export interface AnalyzerStatus {
  status: 'Active' | 'Inactive' | 'Maintenance';
}

export interface Protocol {
  protocol: 'Astm' | 'Hl7';
}

export interface Analyzer {
  id: string;
  name: string;
  model: string;
  serialNumber?: string;
  manufacturer?: string;
  connectionType: ConnectionType;
  ipAddress?: string;
  port?: number;
  comPort?: string;
  baudRate?: number;
  protocol: Protocol;
  status: AnalyzerStatus;
  activateOnStart: boolean;
  createdAt: Date;
  updatedAt: Date;
}

export interface PatientName {
  lastName?: string;
  firstName?: string;
  middleName?: string;
  title?: string;
}

export interface PatientAddress {
  street?: string;
  city?: string;
  state?: string;
  zip?: string;
  countryCode?: string;
}

export interface PatientPhysicians {
  ordering?: string;
  attending?: string;
  referring?: string;
}

export interface PhysicalAttribute {
  value: number;
  unit: string;
}

export interface PhysicalAttributes {
  height?: PhysicalAttribute;
  weight?: PhysicalAttribute;
}

export interface Patient {
  id: string;
  name: PatientName;
  birthDate?: Date;
  sex: 'Male' | 'Female' | 'Other';
  address?: PatientAddress;
  telephone: string[];
  physicians?: PatientPhysicians;
  physicalAttributes?: PhysicalAttributes;
  createdAt: Date;
  updatedAt: Date;
}

export interface ReferenceRange {
  lowerLimit?: number;
  upperLimit?: number;
}

export interface ResultFlags {
  abnormalFlag?: string;
  natureOfAbnormality?: string;
}

export interface TestResultMetadata {
  sequenceNumber: number;
  instrument?: string;
}

export interface TestResult {
  id: string;
  testId: string;
  sampleId: string;
  value: string;
  units?: string;
  referenceRange?: ReferenceRange;
  flags?: ResultFlags;
  status: 'Correction' | 'Final' | 'Preliminary';
  completedDateTime?: Date;
  metadata: TestResultMetadata;
  analyzerId?: string;
  patientId: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface Sample {
  id: string;
  containerInfo?: {
    number: string;
    containerType: string;
  };
  collection?: {
    dateTime?: Date;
    collectorId?: string;
  };
  reception?: {
    dateTime?: Date;
  };
  sampleType: 'Blood' | 'Urine' | 'Serum' | 'Plasma' | 'Csf' | 'Other';
  status: 'Pending' | 'InProgress' | 'Completed' | 'Canceled' | 'Error';
  position?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface TestOrder {
  id: string;
  sequenceNumber: number;
  specimenId: string;
  tests: Array<{
    universalId: string;
    name: string;
  }>;
  priority: 'Routine' | 'Stat' | 'AsapEmergency';
  actionCode: 'Add' | 'New' | 'Pending' | 'Cancel';
  orderingProvider?: string;
  schedulingInfo?: {
    collectionDate?: Date;
    receivedDate?: Date;
  };
  createdAt: Date;
  updatedAt: Date;
}

export interface UploadStatus {
  id: string;
  resultId: string;
  externalSystemId: string;
  status: 'Pending' | 'Uploading' | 'Uploaded' | 'Failed';
  uploadDate?: Date;
  responseCode?: string;
  responseMessage?: string;
  retryCount: number;
  createdAt: Date;
  updatedAt: Date;
}

export interface SystemEvent {
  id: string;
  timestamp: Date;
  type: 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';
  source: 'ANALYZER' | 'PROTOCOL' | 'DATABASE' | 'UPLOAD' | 'SYSTEM';
  message: string;
  details?: any;
}

export interface DataFlowMetrics {
  totalSamplesProcessed: number;
  samplesInQueue: number;
  successfulUploads: number;
  failedUploads: number;
  averageProcessingTime: number;
  systemUptime: number;
}