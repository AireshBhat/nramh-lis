import { Analyzer, Patient, TestResult, Sample, TestOrder, UploadStatus, SystemEvent, DataFlowMetrics } from './types';

export const mockAnalyzers: Analyzer[] = [
  {
    id: 'analyzer-001',
    name: 'Cobas 8000',
    model: 'c502',
    serialNumber: 'SN-2023-001',
    manufacturer: 'Roche',
    connectionType: { type: 'TcpIp' },
    ipAddress: '192.168.1.100',
    port: 8080,
    protocol: { protocol: 'Astm' },
    status: { status: 'Active' },
    activateOnStart: true,
    createdAt: new Date('2023-01-15'),
    updatedAt: new Date('2024-01-15')
  },
  {
    id: 'analyzer-002',
    name: 'Architect i1000SR',
    model: 'i1000SR',
    serialNumber: 'SN-2023-002',
    manufacturer: 'Abbott',
    connectionType: { type: 'Serial' },
    comPort: 'COM3',
    baudRate: 9600,
    protocol: { protocol: 'Hl7' },
    status: { status: 'Maintenance' },
    activateOnStart: false,
    createdAt: new Date('2023-02-20'),
    updatedAt: new Date('2024-01-14')
  }
];

export const mockPatients: Patient[] = [
  {
    id: 'PAT-001',
    name: {
      firstName: 'John',
      lastName: 'Doe',
      middleName: 'Michael'
    },
    birthDate: new Date('1985-05-15'),
    sex: 'Male',
    address: {
      street: '123 Main St',
      city: 'New York',
      state: 'NY',
      zip: '10001',
      countryCode: 'US'
    },
    telephone: ['+1-555-123-4567'],
    physicians: {
      ordering: 'Dr. Smith',
      attending: 'Dr. Johnson'
    },
    physicalAttributes: {
      height: { value: 180, unit: 'cm' },
      weight: { value: 75, unit: 'kg' }
    },
    createdAt: new Date('2024-01-10'),
    updatedAt: new Date('2024-01-15')
  },
  {
    id: 'PAT-002',
    name: {
      firstName: 'Jane',
      lastName: 'Smith',
      title: 'Mrs.'
    },
    birthDate: new Date('1990-08-22'),
    sex: 'Female',
    address: {
      street: '456 Oak Ave',
      city: 'Los Angeles',
      state: 'CA',
      zip: '90001',
      countryCode: 'US'
    },
    telephone: ['+1-555-987-6543'],
    physicians: {
      ordering: 'Dr. Williams',
      attending: 'Dr. Brown'
    },
    physicalAttributes: {
      height: { value: 165, unit: 'cm' },
      weight: { value: 60, unit: 'kg' }
    },
    createdAt: new Date('2024-01-12'),
    updatedAt: new Date('2024-01-15')
  }
];

export const mockTestResults: TestResult[] = [
  {
    id: 'RESULT-001',
    testId: '^^^ALB',
    sampleId: 'SAMPLE-001',
    value: '4.2',
    units: 'g/dL',
    referenceRange: {
      lowerLimit: 3.5,
      upperLimit: 5.0
    },
    flags: {
      abnormalFlag: 'N'
    },
    status: 'Final',
    completedDateTime: new Date('2024-01-15T10:30:00'),
    metadata: {
      sequenceNumber: 1,
      instrument: 'Cobas 8000'
    },
    analyzerId: 'analyzer-001',
    createdAt: new Date('2024-01-15T10:30:00'),
    updatedAt: new Date('2024-01-15T10:30:00')
  },
  {
    id: 'RESULT-002',
    testId: '^^^GLU',
    sampleId: 'SAMPLE-001',
    value: '95',
    units: 'mg/dL',
    referenceRange: {
      lowerLimit: 70,
      upperLimit: 100
    },
    flags: {
      abnormalFlag: 'N'
    },
    status: 'Final',
    completedDateTime: new Date('2024-01-15T10:31:00'),
    metadata: {
      sequenceNumber: 2,
      instrument: 'Cobas 8000'
    },
    analyzerId: 'analyzer-001',
    createdAt: new Date('2024-01-15T10:31:00'),
    updatedAt: new Date('2024-01-15T10:31:00')
  }
];

export const mockSamples: Sample[] = [
  {
    id: 'SAMPLE-001',
    containerInfo: {
      number: 'TUBE-001',
      containerType: 'Tube5to7ml'
    },
    collection: {
      dateTime: new Date('2024-01-15T08:00:00'),
      collectorId: 'TECH-001'
    },
    reception: {
      dateTime: new Date('2024-01-15T09:00:00')
    },
    sampleType: 'Blood',
    status: 'Completed',
    position: 'A1',
    createdAt: new Date('2024-01-15T08:00:00'),
    updatedAt: new Date('2024-01-15T10:30:00')
  }
];

export const mockUploadStatus: UploadStatus[] = [
  {
    id: 'UPLOAD-001',
    resultId: 'RESULT-001',
    externalSystemId: 'HIS-MAIN',
    status: 'Uploaded',
    uploadDate: new Date('2024-01-15T10:35:00'),
    responseCode: '200',
    responseMessage: 'Success',
    retryCount: 0,
    createdAt: new Date('2024-01-15T10:32:00'),
    updatedAt: new Date('2024-01-15T10:35:00')
  },
  {
    id: 'UPLOAD-002',
    resultId: 'RESULT-002',
    externalSystemId: 'HIS-MAIN',
    status: 'Failed',
    uploadDate: new Date('2024-01-15T10:36:00'),
    responseCode: '500',
    responseMessage: 'Internal Server Error',
    retryCount: 2,
    createdAt: new Date('2024-01-15T10:33:00'),
    updatedAt: new Date('2024-01-15T10:36:00')
  }
];

export const mockSystemEvents: SystemEvent[] = [
  {
    id: 'EVENT-001',
    timestamp: new Date('2024-01-15T10:30:00'),
    type: 'SUCCESS',
    source: 'ANALYZER',
    message: 'Sample SAMPLE-001 processed successfully',
    details: { sampleId: 'SAMPLE-001', analyzerId: 'analyzer-001' }
  },
  {
    id: 'EVENT-002',
    timestamp: new Date('2024-01-15T10:31:00'),
    type: 'INFO',
    source: 'PROTOCOL',
    message: 'ASTM protocol message received and parsed',
    details: { protocol: 'ASTM', messageType: 'RESULT' }
  },
  {
    id: 'EVENT-003',
    timestamp: new Date('2024-01-15T10:32:00'),
    type: 'SUCCESS',
    source: 'DATABASE',
    message: 'Test results stored in local database',
    details: { resultIds: ['RESULT-001', 'RESULT-002'] }
  },
  {
    id: 'EVENT-004',
    timestamp: new Date('2024-01-15T10:35:00'),
    type: 'SUCCESS',
    source: 'UPLOAD',
    message: 'Result RESULT-001 uploaded to HIS successfully',
    details: { resultId: 'RESULT-001', externalSystemId: 'HIS-MAIN' }
  },
  {
    id: 'EVENT-005',
    timestamp: new Date('2024-01-15T10:36:00'),
    type: 'ERROR',
    source: 'UPLOAD',
    message: 'Failed to upload RESULT-002 to HIS',
    details: { resultId: 'RESULT-002', error: 'Connection timeout' }
  }
];

export const mockMetrics: DataFlowMetrics = {
  totalSamplesProcessed: 1247,
  samplesInQueue: 3,
  successfulUploads: 1156,
  failedUploads: 12,
  averageProcessingTime: 45.7,
  systemUptime: 99.2
};