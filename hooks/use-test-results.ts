import { useState, useEffect, useCallback } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useToast } from '@/hooks/use-toast';
import { Patient, TestResult, ReferenceRange } from '@/lib/types';

// Types for the lab result event payload from backend
interface BackendPatientData {
  id: string;
  name: string;
  birth_date?: string;
  sex?: string;
  address?: string;
  telephone?: string;
  physicians?: string;
  height?: string;
  weight?: string;
}

interface BackendTestResult {
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
}

interface LabResultEventPayload {
  analyzer_id: string;
  patient_id?: string;
  patient_data?: BackendPatientData;
  test_results: BackendTestResult[];
  timestamp: string;
}

// Extended TestResult type for our hook that can handle both string and object reference ranges
interface ExtendedTestResult extends Omit<TestResult, 'referenceRange'> {
  referenceRange?: string | ReferenceRange;
}

interface UseTestResultsReturn {
  latestResults: {
    patientId?: string;
    patientData?: BackendPatientData;
    testResults: ExtendedTestResult[];
    timestamp: Date;
  } | null;
  allResults: Array<{
    patientId?: string;
    patientData?: BackendPatientData;
    testResults: ExtendedTestResult[];
    timestamp: Date;
  }>;
  clearResults: () => void;
}

export function useTestResults(): UseTestResultsReturn {
  const [latestResults, setLatestResults] = useState<UseTestResultsReturn['latestResults']>(null);
  const [allResults, setAllResults] = useState<UseTestResultsReturn['allResults']>([]);
  const { toast } = useToast();

  const handleLabResults = useCallback((event: LabResultEventPayload) => {
    console.log('🔬 New lab results received:', event);
    
    // Convert backend test results to frontend format
    const convertedTestResults: ExtendedTestResult[] = event.test_results.map(backendResult => ({
      id: backendResult.id,
      testId: backendResult.test_id,
      sampleId: backendResult.sample_id,
      value: backendResult.value,
      units: backendResult.units,
      referenceRange: backendResult.reference_range || undefined,
      flags: {
        abnormalFlag: backendResult.flags[0] || undefined,
        natureOfAbnormality: undefined,
      },
      status: backendResult.status as 'Correction' | 'Final' | 'Preliminary',
      completedDateTime: backendResult.completed_date_time ? new Date(backendResult.completed_date_time) : undefined,
      metadata: {
        sequenceNumber: 0, // Could parse from sample_id if needed
        instrument: undefined,
      },
      analyzerId: backendResult.analyzer_id,
      createdAt: new Date(backendResult.created_at),
      updatedAt: new Date(backendResult.updated_at),
    }));
    
    const results = {
      patientId: event.patient_id,
      patientData: event.patient_data,
      testResults: convertedTestResults,
      timestamp: new Date(event.timestamp),
    };

    // Update latest results
    setLatestResults(results);
    
    // Add to all results history
    setAllResults(prev => [...prev, results]);

    // Log detailed information
    console.group('📊 Lab Results Details');
    console.log('Analyzer ID:', event.analyzer_id);
    console.log('Patient ID:', event.patient_id || 'Not provided');
    
    // Log patient details if available
    if (event.patient_data) {
      console.group('👤 Patient Details');
      console.log('Name:', event.patient_data.name);
      console.log('Birth Date:', event.patient_data.birth_date || 'Not provided');
      console.log('Sex:', event.patient_data.sex || 'Not provided');
      console.log('Address:', event.patient_data.address || 'Not provided');
      console.log('Telephone:', event.patient_data.telephone || 'Not provided');
      console.log('Physicians:', event.patient_data.physicians || 'Not provided');
      console.log('Height:', event.patient_data.height || 'Not provided');
      console.log('Weight:', event.patient_data.weight || 'Not provided');
      console.groupEnd();
    }
    
    console.log('Timestamp:', new Date(event.timestamp).toLocaleString());
    console.log('Number of tests:', event.test_results.length);
    
    // Log each test result
    event.test_results.forEach((test, index) => {
      console.group(`🧪 Test ${index + 1}: ${test.test_id}`);
      console.log('Test ID:', test.test_id);
      console.log('Sample ID:', test.sample_id);
      console.log('Value:', test.value);
      console.log('Units:', test.units || 'N/A');
      console.log('Reference Range:', test.reference_range || 'N/A');
      console.log('Status:', test.status);
      console.log('Flags:', test.flags || 'None');
      console.log('Completed:', test.completed_date_time ? new Date(test.completed_date_time).toLocaleString() : 'N/A');
      console.groupEnd();
    });
    console.groupEnd();

    // Show toast notification
    toast({
      title: "New Test Results",
      description: `Received ${event.test_results.length} test results${event.patient_id ? ` for patient ${event.patient_id}` : ''}`,
      variant: "default",
    });
  }, [toast]);

  const clearResults = useCallback(() => {
    setLatestResults(null);
    setAllResults([]);
    console.log('🧹 Test results cleared');
  }, []);

  // Listen for lab result events
  useEffect(() => {
    const unlisten = listen('meril:lab-results', (event) => {
      const payload = event.payload as LabResultEventPayload;
      handleLabResults(payload);
    });

    console.log('🎧 Listening for lab results events...');

    return () => {
      unlisten.then(fn => fn());
    };
  }, [handleLabResults]);

  return {
    latestResults,
    allResults,
    clearResults,
  };
} 