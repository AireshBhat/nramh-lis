import { useState, useEffect, useCallback } from 'react';
import { fetchMerilConfig, convertAnalyzerResponse, MerilConfigResponse, startMerilService } from '@/lib/tauri-commands';
import { Analyzer } from '@/lib/types';

interface UseMerilAnalyzerReturn {
  analyzer: Analyzer | null;
  loading: boolean;
  error: string | null;
  refreshAnalyzer: () => Promise<void>;
  startService: () => Promise<void>;
}

export function useMerilAnalyzer(): UseMerilAnalyzerReturn {
  const [analyzer, setAnalyzer] = useState<Analyzer | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAnalyzer = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const response: MerilConfigResponse = await fetchMerilConfig();
      
      if (response.success && response.analyzer) {
        const convertedAnalyzer = convertAnalyzerResponse(response.analyzer);
        setAnalyzer(convertedAnalyzer);
      } else {
        setError(response.error_message || 'Failed to fetch Meril analyzer configuration');
        setAnalyzer(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch Meril analyzer');
      console.error('Error fetching Meril analyzer:', err);
      setAnalyzer(null);
    } finally {
      setLoading(false);
    }
  }, []);

  const refreshAnalyzer = useCallback(async () => {
    await fetchAnalyzer();
  }, [fetchAnalyzer]);

  const startService = useCallback(async () => {
    try {
      setError(null);
      await startMerilService();
      // Refresh the analyzer status after starting the service
      await fetchAnalyzer();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to start Meril service';
      setError(errorMessage);
      console.error('Error starting Meril service:', err);
      throw err;
    }
  }, [fetchAnalyzer]);

  // Initial fetch
  useEffect(() => {
    fetchAnalyzer();
  }, [fetchAnalyzer]);

  return {
    analyzer,
    loading,
    error,
    refreshAnalyzer,
    startService,
  };
} 