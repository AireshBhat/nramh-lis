'use client';

import { useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useToast } from '@/hooks/use-toast';
import { PatientRepository } from '@/lib/database/repositories/patients';
import { TestResultRepository } from '@/lib/database/repositories/test-results';
import { getDatabaseClient } from '@/lib/database/client';
import { CreatePatientDTO, CreateTestResultDTO } from '@/lib/database/types';
import { initializeDatabaseWithVerification, isDatabaseReady } from '@/lib/database/init';

// Types for the lab results event payload
interface LabResultsEventPayload {
  analyzer_id: string;
  patient_id: string;
  patient_data?: {
    id: string;
    name: string;
    birth_date?: string;
    sex?: string;
    address?: string;
    telephone?: string;
    physicians?: string;
    height?: string;
    weight?: string;
  };
  test_results: Array<{
    id: string;
    test_id: string;
    sample_id: string;
    value: string;
    units?: string;
    reference_range?: string;
    flags: string[];
    status: string;
    completed_date_time?: string;
    analyzer_id?: string;
    created_at: string;
    updated_at: string;
  }>;
  timestamp: string;
}

/**
 * High-level event listener for lab results
 * Listens to meril:lab-results events and populates the database
 */
export function LabResultsListener() {
  const { toast } = useToast();
  const isInitialized = useRef(false);

  useEffect(() => {
    if (isInitialized.current) return;
    isInitialized.current = true;

    const initializeEventListeners = async () => {
      try {
        // Initialize database first
        console.log('Initializing database for lab results listener...');
        const dbStatus = await initializeDatabaseWithVerification();
        
        if (!dbStatus.initialized || !dbStatus.tablesExist) {
          const errorMsg = dbStatus.error || 'Database initialization failed';
          console.error('Database not ready for lab results processing:', errorMsg);
          toast({
            title: "Database Error",
            description: errorMsg,
            variant: "destructive",
          });
          return;
        }

        console.log('Database initialized successfully for lab results processing');

        // Listen for lab results events
        const unlisten = await listen<LabResultsEventPayload>('meril:lab-results', async (event) => {
          console.log('Lab results received:', event.payload);
          
          // Check if database is still ready before processing
          if (!isDatabaseReady()) {
            console.error('Database not ready, skipping lab results processing');
            toast({
              title: "Database Error",
              description: "Database not ready, lab results not processed",
              variant: "destructive",
            });
            return;
          }
          
          try {
            await processLabResults(event.payload);
            
            // Show success toast
            toast({
              title: "New Lab Results",
              description: `Processed ${event.payload.test_results.length} test results for ${event.payload.patient_data?.name || 'Unknown Patient'}`,
              variant: "default",
            });
          } catch (error) {
            console.error('Error processing lab results:', error);
            
            // Show error toast with more specific error message
            let errorMessage = 'Failed to process lab results';
            if (error instanceof Error) {
              if (error.message.includes('Database not initialized')) {
                errorMessage = 'Database not initialized. Please restart the application.';
              } else if (error.message.includes('Database not ready')) {
                errorMessage = 'Database not ready. Please check database connection.';
              } else {
                errorMessage = error.message;
              }
            }
            
            toast({
              title: "Lab Results Error",
              description: errorMessage,
              variant: "destructive",
            });
          }
        });

        console.log('Lab results event listener initialized');

        // Cleanup function
        return () => {
          unlisten();
          console.log('Lab results event listener cleaned up');
        };
      } catch (error) {
        console.error('Failed to initialize lab results event listener:', error);
        toast({
          title: "Event Listener Error",
          description: "Failed to initialize lab results event listener",
          variant: "destructive",
        });
      }
    };

    initializeEventListeners();
  }, [toast]);

  // This component doesn't render anything
  return null;
}

/**
 * Process lab results event and store in database
 */
async function processLabResults(payload: LabResultsEventPayload): Promise<void> {
  // Check if database is ready before processing
  if (!isDatabaseReady()) {
    throw new Error('Database not ready for processing lab results');
  }

  const db = getDatabaseClient();
  const patientRepo = new PatientRepository(db);
  const testResultRepo = new TestResultRepository(db);

  // Use transaction for atomic operations
  await db.transaction(async () => {
    let patientId: string;

    // Process patient data if available
    if (payload.patient_data) {
      try {
        // Check if patient already exists by ID or name
        const existingPatient = await findExistingPatient(
          patientRepo, 
          payload.patient_data.id, 
          payload.patient_data.name
        );
        console.log({ existingPatient })

        if (existingPatient) {
          patientId = existingPatient.id;
          console.log('Using existing patient:', existingPatient.id);
        } else {
          // Create new patient
          const patientData: CreatePatientDTO = {
            id: payload.patient_data.id,
            name: {
              firstName: extractFirstName(payload.patient_data.name, payload.patient_data.id),
              lastName: extractLastName(payload.patient_data.name),
            },
            birthDate: parseBirthDate(payload.patient_data.birth_date),
            sex: mapSexFromString(payload.patient_data.sex),
            telephone: (() => {
              const phone = cleanStringValue('telephone', payload.patient_data.telephone);
              return phone ? [phone] : [];
            })(),
            address: (() => {
              const street = cleanStringValue('address', payload.patient_data.address);
              return street ? { street } : undefined;
            })(),
            physicians: (() => {
              const ordering = cleanStringValue('physicians', payload.patient_data.physicians);
              return ordering ? { ordering } : undefined;
            })(),
            physicalAttributes: (() => {
              const heightValue = parseNumericValue('height', payload.patient_data.height);
              const weightValue = parseNumericValue('weight', payload.patient_data.weight);
              
              if (heightValue === undefined && weightValue === undefined) {
                return undefined;
              }
              
              return {
                height: heightValue !== undefined ? {
                  value: heightValue,
                  unit: 'cm',
                } : undefined,
                weight: weightValue !== undefined ? {
                  value: weightValue,
                  unit: 'kg',
                } : undefined,
              };
            })(),
          };

          const newPatient = await patientRepo.create(patientData);
          console.log({ newPatient })
          patientId = newPatient.id;
          console.log('Created new patient:', newPatient.id);
        }
      } catch (error) {
        console.error('Error processing patient data:', error);
        throw new Error(`Failed to process patient data: ${error instanceof Error ? error.message : 'Unknown error'}`);
      }
    }

    // Process test results
    if (payload.test_results.length > 0) {
      try {
        const testResultData: CreateTestResultDTO[] = payload.test_results.map((result) => ({
          testId: result.test_id,
          sampleId: result.sample_id,
          value: result.value,
          units: result.units,
          referenceRange: result.reference_range ? parseReferenceRange(result.reference_range) : undefined,
          flags: {
            abnormalFlag: result.flags.find(flag => ['H', 'L', 'HH', 'LL', 'N'].includes(flag)) || undefined,
            natureOfAbnormality: result.flags.find(flag => !['H', 'L', 'HH', 'LL', 'N'].includes(flag)) || undefined,
          },
          status: mapStatusFromString(result.status),
          completedDateTime: result.completed_date_time ? new Date(result.completed_date_time) : undefined,
          metadata: {
            sequenceNumber: 1, // Default sequence number
            instrument: payload.analyzer_id,
          },
          patientId: payload.patient_id,
          analyzerId: payload.analyzer_id,
        }));

        // Batch insert test results
        const resultIds = await testResultRepo.batchInsert(testResultData);
        console.log(`Created ${resultIds.length} test results:`, resultIds);
      } catch (error) {
        console.error('Error processing test results:', error);
        throw new Error(`Failed to process test results: ${error instanceof Error ? error.message : 'Unknown error'}`);
      }
    }
  });
}

/**
 * Find existing patient by ID or name
 */
async function findExistingPatient(
  patientRepo: PatientRepository, 
  patientId?: string, 
  patientName?: string
): Promise<any> {
  if (patientId) {
    try {
      const patient = await patientRepo.findById(patientId);
      if (patient) return patient;
    } catch (error) {
      console.log('Patient not found by ID:', patientId);
    }
  }

  if (patientName) {
    try {
      const firstName = extractFirstName(patientName, patientId);
      const lastName = extractLastName(patientName);
      const patients = await patientRepo.searchByName(lastName, firstName, 10);
      if (patients.length > 0) {
        return patients[0]; // Return first match
      }
    } catch (error) {
      console.log('Patient not found by name:', patientName);
    }
  }

  return null;
}

/**
 * Extract first name from full name
 */
function extractFirstName(fullName: string, id?: string): string {
  if (!fullName.trim()) {
    return id ? id.split(/\s+/)[0] || '' : '';
  }
  const parts = fullName.trim().split(/\s+/);
  return parts.length > 1 ? parts[0] : '';
}

/**
 * Extract last name from full name
 */
function extractLastName(fullName: string): string {
  const parts = fullName.trim().split(/\s+/);
  return parts.length > 1 ? parts.slice(1).join(' ') : fullName;
}

/**
 * Validate and parse birth date
 */
function parseBirthDate(birthDate?: string): Date | undefined {
  if (!birthDate || birthDate.trim() === '' || birthDate === 'N') {
    return undefined;
  }

  try {
    // Try to parse the date
    const parsedDate = new Date(birthDate);
    
    // Check if the date is valid (not NaN and not too far in the past/future)
    if (isNaN(parsedDate.getTime())) {
      console.warn('Invalid birth date format:', birthDate);
      return undefined;
    }

    // Check if date is reasonable (not before 1900 or after current year + 10)
    const currentYear = new Date().getFullYear();
    const dateYear = parsedDate.getFullYear();
    
    if (dateYear < 1900 || dateYear > currentYear + 10) {
      console.warn('Birth date out of reasonable range:', birthDate);
      return undefined;
    }

    return parsedDate;
  } catch (error) {
    console.warn('Error parsing birth date:', birthDate, error);
    return undefined;
  }
}

/**
 * Map sex string to database format
 */
function mapSexFromString(sex?: string): 'Male' | 'Female' | 'Other' {
  if (!sex || sex.trim() === '' || sex === 'N') {
    // Since the database requires a NOT NULL value, we'll use 'Other' as default
    // You might want to create a migration to allow NULL values for sex
    return 'Other';
  }
  
  const normalized = sex.toLowerCase().trim();
  if (normalized === 'm' || normalized === 'male') return 'Male';
  if (normalized === 'f' || normalized === 'female') return 'Female';
  
  // For any other value, use 'Other' as default
  return 'Other';
}

/**
 * Map status string to database format
 */
function mapStatusFromString(status: string): 'Correction' | 'Final' | 'Preliminary' {
  const normalized = status.toLowerCase();
  if (normalized === 'c' || normalized === 'correction') return 'Correction';
  if (normalized === 'p' || normalized === 'preliminary') return 'Preliminary';
  return 'Final';
}

/**
 * Parse and validate numeric value
 */
function parseNumericValue(fieldName: string, value?: string): number | undefined {
  if (!value || value.trim() === '' || value === 'N') {
    return undefined;
  }

  try {
    const parsed = parseFloat(value);
    if (isNaN(parsed)) {
      console.warn(`Invalid ${fieldName} value:`, value);
      return undefined;
    }
    return parsed;
  } catch (error) {
    console.warn(`Error parsing ${fieldName}:`, value, error);
    return undefined;
  }
}

/**
 * Clean and validate string value
 */
function cleanStringValue(fieldName: string, value?: string): string | undefined {
  if (!value || value.trim() === '' || value === 'N') {
    return undefined;
  }
  
  const cleaned = value.trim();
  if (cleaned.length === 0) {
    return undefined;
  }
  
  return cleaned;
}

/**
 * Parse reference range string
 */
function parseReferenceRange(rangeStr: string): { lowerLimit?: number; upperLimit?: number } {
  try {
    // Handle common formats: "3.5-5.0", ">3.5", "<5.0", etc.
    const cleanRange = rangeStr.trim();
    
    if (cleanRange.includes('-')) {
      const [lower, upper] = cleanRange.split('-').map(s => s.trim());
      return {
        lowerLimit: lower ? parseFloat(lower) : undefined,
        upperLimit: upper ? parseFloat(upper) : undefined,
      };
    } else if (cleanRange.startsWith('>')) {
      const value = parseFloat(cleanRange.substring(1));
      return { lowerLimit: value };
    } else if (cleanRange.startsWith('<')) {
      const value = parseFloat(cleanRange.substring(1));
      return { upperLimit: value };
    } else {
      const value = parseFloat(cleanRange);
      return isNaN(value) ? {} : { lowerLimit: value, upperLimit: value };
    }
  } catch (error) {
    console.warn('Failed to parse reference range:', rangeStr, error);
    return {};
  }
} 