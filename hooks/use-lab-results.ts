import { useState, useEffect, useCallback, useMemo } from 'react';
import { useTestResults } from './use-test-results';
import { usePatients } from './use-patients';
import { TestResult } from '@/lib/types';

interface UseLabResultsReturn {
  // Data
  latestResults: TestResult | null;
  allResults: TestResult[];
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
  
  // State for lab results
  const [labResults, setLabResults] = useState<TestResult[]>([]);
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
   * Process and sort test results by timestamp (newest first)
   */
  const processTestResults = useCallback((testResults: TestResult[]): TestResult[] => {
    return testResults
      .sort((a, b) => {
        const aTime = a.completedDateTime || a.createdAt;
        const bTime = b.completedDateTime || b.createdAt;
        return bTime.getTime() - aTime.getTime();
      })
      .slice(0, maxResultsInMemory);
  }, [maxResultsInMemory]);

  // Latest results (most recent test result)
  const latestResults = useMemo(() => {
    return labResults.length > 0 ? labResults[0] : null;
  }, [labResults]);

  // All results (all test results)
  const allResults = useMemo(() => {
    return labResults;
  }, [labResults]);

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
    setLabResults([]);
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

  // Update results when test results change
  useEffect(() => {
    if (testResults.length > 0) {
      const processedResults = processTestResults(testResults);
      setLabResults(processedResults);
      console.log(`Processed ${testResults.length} test results, showing ${processedResults.length} most recent`);
    } else {
      setLabResults([]);
    }
  }, [testResults, processTestResults]);

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