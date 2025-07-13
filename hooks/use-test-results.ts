import { useState, useEffect, useCallback, useMemo } from 'react';
import { TestResult } from '@/lib/types';
import { TestResultRepository } from '@/lib/database/repositories/test-results';
import { getDatabaseClient } from '@/lib/database/client';
import { ValidationError, CreateTestResultDTO, UpdateTestResultDTO } from '@/lib/database/types';

interface UseTestResultsOptions {
  autoLoad?: boolean;
  limit?: number;
}

interface UseTestResultsReturn {
  // Data
  testResults: TestResult[];
  testResult: TestResult | null;
  loading: boolean;
  error: string | null;
  
  // Statistics
  statistics: {
    total: number;
    final: number;
    preliminary: number;
    correction: number;
    abnormal: number;
    today: number;
  };
  
  // Actions
  fetchTestResults: (options?: { limit?: number; offset?: number }) => Promise<void>;
  getTestResult: (id: string) => Promise<TestResult | null>;
  createTestResult: (data: CreateTestResultDTO) => Promise<TestResult>;
  updateTestResult: (id: string, data: UpdateTestResultDTO) => Promise<TestResult>;
  deleteTestResult: (id: string) => Promise<boolean>;
  
  // Specialized queries
  findBySampleId: (sampleId: string) => Promise<TestResult[]>;
  findByAnalyzerId: (analyzerId: string) => Promise<TestResult[]>;
  findByDateRange: (startDate: Date, endDate: Date) => Promise<TestResult[]>;
  findAbnormalResults: (limit?: number) => Promise<TestResult[]>;
  findByStatus: (status: 'Correction' | 'Final' | 'Preliminary') => Promise<TestResult[]>;
  findRecentResults: (hours?: number) => Promise<TestResult[]>;
  batchInsert: (results: CreateTestResultDTO[]) => Promise<string[]>;
  
  // Utilities
  clearError: () => void;
  refresh: () => Promise<void>;
  refreshStatistics: () => Promise<void>;
}

/**
 * React hook for test result database operations
 * Provides specialized functionality for managing laboratory test results
 */
export function useTestResults(options: UseTestResultsOptions = {}): UseTestResultsReturn {
  const { autoLoad = true, limit = 100 } = options;
  
  // State
  const [testResults, setTestResults] = useState<TestResult[]>([]);
  const [testResult, setTestResult] = useState<TestResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [statistics, setStatistics] = useState({
    total: 0,
    final: 0,
    preliminary: 0,
    correction: 0,
    abnormal: 0,
    today: 0
  });

  // Repository instance
  const repository = useMemo(() => {
    const db = getDatabaseClient();
    return new TestResultRepository(db);
  }, []);

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  /**
   * Fetch test results with pagination
   */
  const fetchTestResults = useCallback(async (fetchOptions?: { limit?: number; offset?: number }) => {
    try {
      setLoading(true);
      setError(null);

      const result = await repository.findAll({
        limit: fetchOptions?.limit || limit,
        offset: fetchOptions?.offset || 0,
        orderBy: 'completed_date_time',
        orderDirection: 'DESC'
      });

      setTestResults(result.data);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch test results';
      setError(errorMessage);
      console.error('Error fetching test results:', err);
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Get a single test result by ID
   */
  const getTestResult = useCallback(async (id: string): Promise<TestResult | null> => {
    try {
      setLoading(true);
      setError(null);

      const foundTestResult = await repository.findById(id);
      setTestResult(foundTestResult);
      return foundTestResult;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get test result';
      setError(errorMessage);
      console.error('Error getting test result:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository]);

  /**
   * Create a new test result
   */
  const createTestResult = useCallback(async (data: CreateTestResultDTO): Promise<TestResult> => {
    try {
      setLoading(true);
      setError(null);

      const newTestResult = await repository.create(data);
      
      // Add to the beginning of the list
      setTestResults(prev => [newTestResult, ...prev]);
      
      // Refresh statistics
      await refreshStatistics();

      return newTestResult;
    } catch (err) {
      let errorMessage = 'Failed to create test result';
      
      if (err instanceof ValidationError) {
        errorMessage = `${err.field}: ${err.message}`;
      } else if (err instanceof Error) {
        errorMessage = err.message;
      }
      
      setError(errorMessage);
      console.error('Error creating test result:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository]);

  /**
   * Update an existing test result
   */
  const updateTestResult = useCallback(async (id: string, data: UpdateTestResultDTO): Promise<TestResult> => {
    try {
      setLoading(true);
      setError(null);

      const updatedTestResult = await repository.update(id, data);
      
      // Update in the list
      setTestResults(prev => prev.map(tr => tr.id === id ? updatedTestResult : tr));
      
      // Update single test result if it's the current one
      if (testResult?.id === id) {
        setTestResult(updatedTestResult);
      }

      return updatedTestResult;
    } catch (err) {
      let errorMessage = 'Failed to update test result';
      
      if (err instanceof ValidationError) {
        errorMessage = `${err.field}: ${err.message}`;
      } else if (err instanceof Error) {
        errorMessage = err.message;
      }
      
      setError(errorMessage);
      console.error('Error updating test result:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, testResult]);

  /**
   * Delete a test result
   */
  const deleteTestResult = useCallback(async (id: string): Promise<boolean> => {
    try {
      setLoading(true);
      setError(null);

      const success = await repository.delete(id);
      
      if (success) {
        // Remove from the list
        setTestResults(prev => prev.filter(tr => tr.id !== id));
        
        // Clear single test result if it's the deleted one
        if (testResult?.id === id) {
          setTestResult(null);
        }
        
        // Refresh statistics
        await refreshStatistics();
      }

      return success;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to delete test result';
      setError(errorMessage);
      console.error('Error deleting test result:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, testResult]);

  /**
   * Find test results by sample ID
   */
  const findBySampleId = useCallback(async (sampleId: string): Promise<TestResult[]> => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.findBySampleId(sampleId);
      setTestResults(results);
      return results;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to find test results by sample ID';
      setError(errorMessage);
      console.error('Error finding test results by sample ID:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository]);

  /**
   * Find test results by analyzer ID
   */
  const findByAnalyzerId = useCallback(async (analyzerId: string): Promise<TestResult[]> => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.findByAnalyzerId(analyzerId, limit);
      setTestResults(results);
      return results;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to find test results by analyzer ID';
      setError(errorMessage);
      console.error('Error finding test results by analyzer ID:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Find test results by date range
   */
  const findByDateRange = useCallback(async (startDate: Date, endDate: Date): Promise<TestResult[]> => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.findByDateRange(startDate, endDate, limit);
      setTestResults(results);
      return results;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to find test results by date range';
      setError(errorMessage);
      console.error('Error finding test results by date range:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Find abnormal test results
   */
  const findAbnormalResults = useCallback(async (abnormalLimit: number = 50): Promise<TestResult[]> => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.findAbnormalResults(abnormalLimit);
      setTestResults(results);
      return results;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to find abnormal test results';
      setError(errorMessage);
      console.error('Error finding abnormal test results:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository]);

  /**
   * Find test results by status
   */
  const findByStatus = useCallback(async (status: 'Correction' | 'Final' | 'Preliminary'): Promise<TestResult[]> => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.findByStatus(status, limit);
      setTestResults(results);
      return results;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to find test results by status';
      setError(errorMessage);
      console.error('Error finding test results by status:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Find recent test results
   */
  const findRecentResults = useCallback(async (hours: number = 24): Promise<TestResult[]> => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.findRecentResults(hours, limit);
      setTestResults(results);
      return results;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to find recent test results';
      setError(errorMessage);
      console.error('Error finding recent test results:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Batch insert multiple test results
   */
  const batchInsert = useCallback(async (results: CreateTestResultDTO[]): Promise<string[]> => {
    try {
      setLoading(true);
      setError(null);

      const ids = await repository.batchInsert(results);
      
      // Refresh the list and statistics
      await Promise.all([
        fetchTestResults({ limit, offset: 0 }),
        refreshStatistics()
      ]);

      return ids;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to batch insert test results';
      setError(errorMessage);
      console.error('Error batch inserting test results:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, fetchTestResults, limit]);

  /**
   * Refresh statistics
   */
  const refreshStatistics = useCallback(async () => {
    try {
      const stats = await repository.getStatistics();
      setStatistics(stats);
    } catch (err) {
      console.error('Error refreshing statistics:', err);
    }
  }, [repository]);

  /**
   * Refresh the current data
   */
  const refresh = useCallback(async () => {
    await Promise.all([
      fetchTestResults({ limit, offset: 0 }),
      refreshStatistics()
    ]);
  }, [fetchTestResults, refreshStatistics, limit]);

  // Auto-load test results and statistics on mount
  useEffect(() => {
    if (autoLoad) {
      refresh();
    }
  }, [autoLoad, refresh]);

  return {
    // Data
    testResults,
    testResult,
    loading,
    error,
    
    // Statistics
    statistics,
    
    // Actions
    fetchTestResults,
    getTestResult,
    createTestResult,
    updateTestResult,
    deleteTestResult,
    
    // Specialized queries
    findBySampleId,
    findByAnalyzerId,
    findByDateRange,
    findAbnormalResults,
    findByStatus,
    findRecentResults,
    batchInsert,
    
    // Utilities
    clearError,
    refresh,
    refreshStatistics,
  };
} 