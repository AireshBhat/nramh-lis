import { useState, useEffect, useCallback, useMemo } from 'react';
import { useTestResults } from './use-test-results';
import { usePatients } from './use-patients';
import { TestResult } from '@/lib/types';
import { Patient } from '@/lib/types';
import { initializeDatabaseWithVerification } from '@/lib/database/init';

// Types for lab results batch
interface LabResultsBatch {
  id: string;
  timestamp: Date;
  analyzerId: string;
  patientId?: string;
  patientData?: {
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
  testResults: Array<{
    id: string;
    testId: string;
    sampleId: string;
    value: string;
    units?: string;
    referenceRange?: string;
    flags: string[];
    status: string;
    completedDateTime?: string;
    analyzerId?: string;
  }>;
}

interface UseLabResultsReturn {
  // Data
  latestResults: LabResultsBatch | null;
  allResults: LabResultsBatch[];
  loading: boolean;
  error: string | null;
  
  // Actions
  clearResults: () => void;
  refreshResults: () => Promise<void>;
  
  // Utilities
  clearError: () => void;
}

interface UseLabResultsOptions {
  maxResultsInMemory?: number;
  autoRefresh?: boolean;
  refreshInterval?: number; // in milliseconds
}

/**
 * React hook for managing lab results by fetching from database
 * Provides the interface expected by the results page
 */
export function useLabResults(options: UseLabResultsOptions = {}): UseLabResultsReturn {
  const { 
    maxResultsInMemory = 10, 
    autoRefresh = true, 
    refreshInterval = 5000 // 5 seconds default
  } = options;
  
  // State for lab results batches
  const [labResultsBatches, setLabResultsBatches] = useState<LabResultsBatch[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Database hooks for fetching existing data
  const { testResults, fetchTestResults, loading: dbLoading } = useTestResults({ 
    autoLoad: false, 
    limit: 100 
  });
  const { patients, fetchPatients, loading: patientsLoading } = usePatients({ 
    autoLoad: false, 
    limit: 100 
  });

  /**
   * Convert database entities to LabResultsBatch format
   */
  const convertToLabResultsBatches = useCallback((testResults: TestResult[], patients: Patient[]): LabResultsBatch[] => {
    // Group test results by sample ID and analyzer ID to create batches
    const batchMap = new Map<string, LabResultsBatch>();
    
    testResults.forEach((result) => {
      const batchKey = `${result.sampleId}-${result.analyzerId || 'unknown'}`;
      
      if (!batchMap.has(batchKey)) {
        const batch: LabResultsBatch = {
          id: batchKey,
          timestamp: result.completedDateTime || result.createdAt,
          analyzerId: result.analyzerId || 'unknown',
          patientId: result.patientId, // No direct patient ID in test results
          patientData: undefined, // Will be populated if we can find patient by sample ID
          testResults: [],
        };
        
        batchMap.set(batchKey, batch);
      }
      
      const batch = batchMap.get(batchKey)!;
      
      // Convert test result to batch format
      batch.testResults.push({
        id: result.id,
        testId: result.testId,
        sampleId: result.sampleId,
        value: result.value,
        units: result.units,
        referenceRange: result.referenceRange ? 
          `${result.referenceRange.lowerLimit || ''}-${result.referenceRange.upperLimit || ''}` : undefined,
        flags: [
          ...(result.flags?.abnormalFlag ? [result.flags.abnormalFlag] : []),
          ...(result.flags?.natureOfAbnormality ? [result.flags.natureOfAbnormality] : [])
        ],
        status: result.status,
        completedDateTime: result.completedDateTime?.toISOString(),
        analyzerId: result.analyzerId,
      });
    });
    
    // Convert map to array and sort by timestamp (newest first)
    const batches = Array.from(batchMap.values())
      .sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
      .slice(0, maxResultsInMemory);
    
    return batches;
  }, [maxResultsInMemory]);

  // Latest results (most recent batch)
  const latestResults = useMemo(() => {
    return labResultsBatches.length > 0 ? labResultsBatches[0] : null;
  }, [labResultsBatches]);

  // All results (all batches)
  const allResults = useMemo(() => {
    return labResultsBatches;
  }, [labResultsBatches]);

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  /**
   * Clear all results from memory
   */
  const clearResults = useCallback(() => {
    setLabResultsBatches([]);
    console.log('Lab results cleared from memory');
  }, []);

  /**
   * Refresh results from database
   */
  const refreshResults = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      // Fetch recent test results and patients from database
      await Promise.all([
        fetchTestResults({ limit: 100, offset: 0 }),
        fetchPatients({ limit: 100, offset: 0 })
      ]);

      console.log('Lab results refreshed from database');
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to refresh lab results';
      setError(errorMessage);
      console.error('Error refreshing lab results:', err);
    } finally {
      setLoading(false);
    }
  }, [fetchTestResults, fetchPatients]);

  // Update batches when test results or patients change
  useEffect(() => {
    if (testResults.length > 0 || patients.length > 0) {
      const batches = convertToLabResultsBatches(testResults, patients);
      setLabResultsBatches(batches);
      console.log(`Converted ${testResults.length} test results into ${batches.length} batches`);
    }
  }, [testResults, patients, convertToLabResultsBatches]);

  // Initialize database and auto-refresh on mount if enabled
  useEffect(() => {
    const initializeAndRefresh = async () => {
      try {
        // Initialize database first
        console.log('Initializing database for lab results hook...');
        const dbStatus = await initializeDatabaseWithVerification();
        
        if (!dbStatus.initialized || !dbStatus.tablesExist) {
          const errorMsg = dbStatus.error || 'Database initialization failed';
          console.error('Database not ready for lab results hook:', errorMsg);
          setError(errorMsg);
          return;
        }

        console.log('Database initialized successfully for lab results hook');
        
        // Refresh results if auto-refresh is enabled
        if (autoRefresh) {
          await refreshResults();
        }
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Failed to initialize database';
        console.error('Error initializing database for lab results hook:', error);
        setError(errorMessage);
      }
    };

    initializeAndRefresh();
  }, [autoRefresh, refreshResults]);

  // Set up auto-refresh interval if enabled
  useEffect(() => {
    if (!autoRefresh || refreshInterval <= 0) return;

    const interval = setInterval(() => {
      refreshResults();
    }, refreshInterval);

    return () => clearInterval(interval);
  }, [autoRefresh, refreshInterval, refreshResults]);

  // Combine loading states
  const isLoading = loading || dbLoading || patientsLoading;

  return {
    // Data
    latestResults,
    allResults,
    loading: isLoading,
    error,
    
    // Actions
    clearResults,
    refreshResults,
    
    // Utilities
    clearError,
  };
} 