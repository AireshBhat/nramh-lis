import { useState, useEffect, useCallback, useMemo } from 'react';
import { Patient } from '@/lib/types';
import { PatientRepository } from '@/lib/database/repositories/patients';
import { getDatabaseClient } from '@/lib/database/client';
import { ValidationError, CreatePatientDTO, UpdatePatientDTO } from '@/lib/database/types';

interface UsePatientsOptions {
  autoLoad?: boolean;
  limit?: number;
}

interface UsePatientsReturn {
  // Data
  patients: Patient[];
  patient: Patient | null;
  loading: boolean;
  error: string | null;
  
  // Pagination
  total: number;
  hasMore: boolean;
  
  // Actions
  fetchPatients: (options?: { limit?: number; offset?: number }) => Promise<void>;
  searchPatients: (lastName?: string, firstName?: string) => Promise<void>;
  createPatient: (data: CreatePatientDTO) => Promise<Patient>;
  updatePatient: (id: string, data: UpdatePatientDTO) => Promise<Patient>;
  deletePatient: (id: string) => Promise<boolean>;
  getPatient: (id: string) => Promise<Patient | null>;
  
  // Utilities
  clearError: () => void;
  refresh: () => Promise<void>;
}

/**
 * React hook for patient database operations
 * Provides a clean interface for managing patients in the UI
 */
export function usePatients(options: UsePatientsOptions = {}): UsePatientsReturn {
  const { autoLoad = true, limit = 100 } = options;
  
  // State
  const [patients, setPatients] = useState<Patient[]>([]);
  const [patient, setPatient] = useState<Patient | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [total, setTotal] = useState(0);
  const [currentOffset, setCurrentOffset] = useState(0);

  // Repository instance
  const repository = useMemo(() => {
    const db = getDatabaseClient();
    return new PatientRepository(db);
  }, []);

  // Check if there are more patients to load
  const hasMore = useMemo(() => {
    return patients.length < total;
  }, [patients.length, total]);

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  /**
   * Fetch patients with pagination
   */
  const fetchPatients = useCallback(async (fetchOptions?: { limit?: number; offset?: number }) => {
    try {
      setLoading(true);
      setError(null);

      const result = await repository.findAll({
        limit: fetchOptions?.limit || limit,
        offset: fetchOptions?.offset || 0,
        orderBy: 'created_at',
        orderDirection: 'DESC'
      });

      if (fetchOptions?.offset === 0 || !fetchOptions?.offset) {
        // First page or no offset specified
        setPatients(result.data);
        setCurrentOffset(result.data.length);
      } else {
        // Append to existing data
        setPatients(prev => [...prev, ...result.data]);
        setCurrentOffset(prev => prev + result.data.length);
      }

      setTotal(result.total);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch patients';
      setError(errorMessage);
      console.error('Error fetching patients:', err);
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Search patients by name
   */
  const searchPatients = useCallback(async (lastName?: string, firstName?: string) => {
    try {
      setLoading(true);
      setError(null);

      const results = await repository.searchByName(lastName, firstName, limit);
      setPatients(results);
      setTotal(results.length);
      setCurrentOffset(results.length);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to search patients';
      setError(errorMessage);
      console.error('Error searching patients:', err);
    } finally {
      setLoading(false);
    }
  }, [repository, limit]);

  /**
   * Create a new patient
   */
  const createPatient = useCallback(async (data: CreatePatientDTO): Promise<Patient> => {
    try {
      setLoading(true);
      setError(null);

      const newPatient = await repository.create(data);
      
      // Add to the beginning of the list
      setPatients(prev => [newPatient, ...prev]);
      setTotal(prev => prev + 1);
      setCurrentOffset(prev => prev + 1);

      return newPatient;
    } catch (err) {
      let errorMessage = 'Failed to create patient';
      
      if (err instanceof ValidationError) {
        errorMessage = `${err.field}: ${err.message}`;
      } else if (err instanceof Error) {
        errorMessage = err.message;
      }
      
      setError(errorMessage);
      console.error('Error creating patient:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository]);

  /**
   * Update an existing patient
   */
  const updatePatient = useCallback(async (id: string, data: UpdatePatientDTO): Promise<Patient> => {
    try {
      setLoading(true);
      setError(null);

      const updatedPatient = await repository.update(id, data);
      
      // Update in the list
      setPatients(prev => prev.map(p => p.id === id ? updatedPatient : p));
      
      // Update single patient if it's the current one
      if (patient?.id === id) {
        setPatient(updatedPatient);
      }

      return updatedPatient;
    } catch (err) {
      let errorMessage = 'Failed to update patient';
      
      if (err instanceof ValidationError) {
        errorMessage = `${err.field}: ${err.message}`;
      } else if (err instanceof Error) {
        errorMessage = err.message;
      }
      
      setError(errorMessage);
      console.error('Error updating patient:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, patient]);

  /**
   * Delete a patient
   */
  const deletePatient = useCallback(async (id: string): Promise<boolean> => {
    try {
      setLoading(true);
      setError(null);

      const success = await repository.delete(id);
      
      if (success) {
        // Remove from the list
        setPatients(prev => prev.filter(p => p.id !== id));
        setTotal(prev => prev - 1);
        setCurrentOffset(prev => prev - 1);
        
        // Clear single patient if it's the deleted one
        if (patient?.id === id) {
          setPatient(null);
        }
      }

      return success;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to delete patient';
      setError(errorMessage);
      console.error('Error deleting patient:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository, patient]);

  /**
   * Get a single patient by ID
   */
  const getPatient = useCallback(async (id: string): Promise<Patient | null> => {
    try {
      setLoading(true);
      setError(null);

      const foundPatient = await repository.findById(id);
      setPatient(foundPatient);
      return foundPatient;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get patient';
      setError(errorMessage);
      console.error('Error getting patient:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [repository]);

  /**
   * Refresh the current data
   */
  const refresh = useCallback(async () => {
    await fetchPatients({ limit, offset: 0 });
  }, [fetchPatients, limit]);

  // Auto-load patients on mount
  useEffect(() => {
    if (autoLoad) {
      fetchPatients({ limit, offset: 0 });
    }
  }, [autoLoad, fetchPatients, limit]);

  return {
    // Data
    patients,
    patient,
    loading,
    error,
    
    // Pagination
    total,
    hasMore,
    
    // Actions
    fetchPatients,
    searchPatients,
    createPatient,
    updatePatient,
    deletePatient,
    getPatient,
    
    // Utilities
    clearError,
    refresh,
  };
} 