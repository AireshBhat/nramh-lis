import { useState, useEffect, useCallback } from 'react';
import { 
  fetchBF6900Config, 
  convertAnalyzerResponse, 
  BF6900ConfigResponse, 
  startBF6900Service, 
  stopBF6900Service,
  getBF6900ServiceStatus,
  BF6900ServiceStatus
} from '@/lib/tauri-commands';
import { Analyzer } from '@/lib/types';
import { listen } from '@tauri-apps/api/event';
import { useToast } from '@/hooks/use-toast';

interface useBF6900AnalyzerReturn {
  analyzer: Analyzer | null;
  hl7Settings: any | null;
  serviceStatus: BF6900ServiceStatus | null;
  loading: boolean;
  error: string | null;
  refreshAnalyzer: () => Promise<void>;
  startService: () => Promise<void>;
  stopService: () => Promise<void>;
  refreshServiceStatus: () => Promise<void>;
}

export function useBF6900Analyzer(): useBF6900AnalyzerReturn {
  const [analyzer, setAnalyzer] = useState<Analyzer | null>(null);
  const [hl7Settings, setHl7Settings] = useState<any | null>(null);
  const [serviceStatus, setServiceStatus] = useState<BF6900ServiceStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { toast } = useToast();

  const fetchAnalyzer = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const response: BF6900ConfigResponse = await fetchBF6900Config();
      
      if (response.success && response.analyzer) {
        const convertedAnalyzer = convertAnalyzerResponse(response.analyzer);
        setAnalyzer(convertedAnalyzer);
        setHl7Settings(response.hl7_settings || null);
      } else {
        setError(response.error_message || 'Failed to fetch BF-6900 analyzer configuration');
        setAnalyzer(null);
        setHl7Settings(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch BF-6900 analyzer');
      console.error('Error fetching BF-6900 analyzer:', err);
      setAnalyzer(null);
      setHl7Settings(null);
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchServiceStatus = useCallback(async () => {
    try {
      const status = await getBF6900ServiceStatus();
      setServiceStatus(status);
    } catch (err) {
      console.error('Error fetching BF-6900 service status:', err);
      // Don't set error here as it's not critical
    }
  }, []);

  const refreshAnalyzer = useCallback(async () => {
    await fetchAnalyzer();
  }, [fetchAnalyzer]);

  const refreshServiceStatus = useCallback(async () => {
    await fetchServiceStatus();
  }, [fetchServiceStatus]);

  const startService = useCallback(async () => {
    try {
      setError(null);
      await startBF6900Service();
      toast({
        title: "Service Started",
        description: "BF-6900 analyzer service has been started successfully.",
        variant: "default",
      });
      // Refresh the analyzer status after starting the service
      await fetchAnalyzer();
      await fetchServiceStatus();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to start BF-6900 service';
      setError(errorMessage);
      toast({
        title: "Service Start Failed",
        description: errorMessage,
        variant: "destructive",
      });
      console.error('Error starting BF-6900 service:', err);
      throw err;
    }
  }, [fetchAnalyzer, fetchServiceStatus, toast]);

  const stopService = useCallback(async () => {
    try {
      setError(null);
      await stopBF6900Service();
      toast({
        title: "Service Stopped",
        description: "BF-6900 analyzer service has been stopped successfully.",
        variant: "default",
      });
      // Refresh the analyzer status after stopping the service
      await fetchAnalyzer();
      await fetchServiceStatus();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to stop BF-6900 service';
      setError(errorMessage);
      toast({
        title: "Service Stop Failed",
        description: errorMessage,
        variant: "destructive",
      });
      console.error('Error stopping BF-6900 service:', err);
      throw err;
    }
  }, [fetchAnalyzer, fetchServiceStatus, toast]);

  // Listen for analyzer status updates and service events
  useEffect(() => {
    const unlisteners: Promise<() => void>[] = [];

    // Listen for analyzer status updates
    unlisteners.push(
      listen('bf6900:analyzer-status-updated', (event) => {
        console.log('BF-6900 analyzer status updated:', event.payload);
        toast({
          title: "Analyzer Status Updated",
          description: "The BF-6900 analyzer status has been updated.",
          variant: "default",
        });
        // Refresh the analyzer to get the updated status
        fetchAnalyzer();
        fetchServiceStatus();
      })
    );

    // Listen for service start events
    unlisteners.push(
      listen('bf6900:service-started', (event) => {
        console.log('BF-6900 service started:', event.payload);
        toast({
          title: "Service Started",
          description: "BF-6900 analyzer service has been started.",
          variant: "default",
        });
        // Refresh the analyzer to get the updated status
        fetchAnalyzer();
        fetchServiceStatus();
      })
    );

    // Listen for service stop events
    unlisteners.push(
      listen('bf6900:service-stopped', (event) => {
        console.log('BF-6900 service stopped:', event.payload);
        toast({
          title: "Service Stopped",
          description: "BF-6900 analyzer service has been stopped.",
          variant: "default",
        });
        // Refresh the analyzer to get the updated status
        fetchAnalyzer();
        fetchServiceStatus();
      })
    );

    // Listen for service errors
    unlisteners.push(
      listen('bf6900:service-error', (event) => {
        console.error('BF-6900 service error:', event.payload);
        const payload = event.payload as { error: string };
        setError(payload.error);
        toast({
          title: "Service Error",
          description: payload.error,
          variant: "destructive",
        });
      })
    );

    return () => {
      unlisteners.forEach(unlisten => unlisten.then(fn => fn()));
    };
  }, [fetchAnalyzer, fetchServiceStatus, toast]);

  // Initial fetch
  useEffect(() => {
    fetchAnalyzer();
    fetchServiceStatus();
  }, [fetchAnalyzer, fetchServiceStatus]);

  return {
    analyzer,
    hl7Settings,
    serviceStatus,
    loading,
    error,
    refreshAnalyzer,
    startService,
    stopService,
    refreshServiceStatus,
  };
} 