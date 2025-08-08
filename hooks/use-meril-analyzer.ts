import { useState, useEffect, useCallback } from 'react';
import { fetchMerilConfig, updateMerilConfig, convertAnalyzerResponse, MerilConfigResponse, startMerilService, stopMerilService } from '@/lib/tauri-commands';
import { Analyzer } from '@/lib/types';
import { listen } from '@tauri-apps/api/event';
import { useToast } from '@/hooks/use-toast';

interface UseMerilAnalyzerReturn {
  analyzer: Analyzer | null;
  loading: boolean;
  error: string | null;
  refreshAnalyzer: () => Promise<void>;
  updateConfiguration: (updatedAnalyzer: Partial<Analyzer>) => Promise<void>;
  startService: () => Promise<void>;
  stopService: () => Promise<void>;
}

export function useMerilAnalyzer(): UseMerilAnalyzerReturn {
  const [analyzer, setAnalyzer] = useState<Analyzer | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { toast } = useToast();

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

  const updateConfiguration = useCallback(async (updatedAnalyzer: Partial<Analyzer>) => {
    try {
      setError(null);
      
      // Convert frontend analyzer format to backend format
      const backendAnalyzer = {
        id: analyzer?.id,
        name: updatedAnalyzer.name || analyzer?.name,
        model: updatedAnalyzer.model || analyzer?.model,
        serial_number: updatedAnalyzer.serialNumber || analyzer?.serialNumber,
        manufacturer: updatedAnalyzer.manufacturer || analyzer?.manufacturer,
        connection_type: updatedAnalyzer.connectionType?.type || analyzer?.connectionType.type,
        ip_address: updatedAnalyzer.ipAddress || analyzer?.ipAddress,
        port: updatedAnalyzer.port || analyzer?.port,
        external_ip: updatedAnalyzer.external_ip,
        external_port: updatedAnalyzer.external_port,
        com_port: updatedAnalyzer.comPort || analyzer?.comPort,
        baud_rate: updatedAnalyzer.baudRate || analyzer?.baudRate,
        protocol: updatedAnalyzer.protocol?.protocol || analyzer?.protocol.protocol,
        activate_on_start: updatedAnalyzer.activateOnStart ?? analyzer?.activateOnStart ?? false,
      };

      const response: MerilConfigResponse = await updateMerilConfig(backendAnalyzer);
      
      if (response.success && response.analyzer) {
        const convertedAnalyzer = convertAnalyzerResponse(response.analyzer);
        setAnalyzer(convertedAnalyzer);
        toast({
          title: "Configuration Updated",
          description: "Meril analyzer configuration has been updated successfully.",
          variant: "default",
        });
      } else {
        throw new Error(response.error_message || 'Failed to update configuration');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to update Meril configuration';
      setError(errorMessage);
      toast({
        title: "Configuration Update Failed",
        description: errorMessage,
        variant: "destructive",
      });
      throw err;
    }
  }, [analyzer, toast]);

  const startService = useCallback(async () => {
    try {
      setError(null);
      await startMerilService();
      toast({
        title: "Service Started",
        description: "Meril analyzer service has been started successfully.",
        variant: "default",
      });
      // Refresh the analyzer status after starting the service
      await fetchAnalyzer();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to start Meril service';
      setError(errorMessage);
      toast({
        title: "Service Start Failed",
        description: errorMessage,
        variant: "destructive",
      });
      console.error('Error starting Meril service:', err);
      throw err;
    }
  }, [fetchAnalyzer, toast]);

  const stopService = useCallback(async () => {
    try {
      setError(null);
      await stopMerilService();
      toast({
        title: "Service Stopped",
        description: "Meril analyzer service has been stopped successfully.",
        variant: "default",
      });
      // Refresh the analyzer status after stopping the service
      await fetchAnalyzer();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to stop Meril service';
      setError(errorMessage);
      toast({
        title: "Service Stop Failed",
        description: errorMessage,
        variant: "destructive",
      });
      console.error('Error stopping Meril service:', err);
      throw err;
    }
  }, [fetchAnalyzer, toast]);

  // Listen for analyzer status updates and service events
  useEffect(() => {
    const unlisteners: Promise<() => void>[] = [];

    // Listen for analyzer status updates
    unlisteners.push(
      listen('meril:analyzer-status-updated', (event) => {
        console.log('Analyzer status updated:', event.payload);
        toast({
          title: "Analyzer Status Updated",
          description: "The Meril analyzer status has been updated.",
          variant: "default",
        });
        // Refresh the analyzer to get the updated status
        fetchAnalyzer();
      })
    );

    // Listen for service start events
    unlisteners.push(
      listen('meril:service-started', (event) => {
        console.log('Meril service started:', event.payload);
        toast({
          title: "Service Started",
          description: "Meril analyzer service has been started.",
          variant: "default",
        });
        // Refresh the analyzer to get the updated status
        fetchAnalyzer();
      })
    );

    // Listen for service stop events
    unlisteners.push(
      listen('meril:service-stopped', (event) => {
        console.log('Meril service stopped:', event.payload);
        toast({
          title: "Service Stopped",
          description: "Meril analyzer service has been stopped.",
          variant: "default",
        });
        // Refresh the analyzer to get the updated status
        fetchAnalyzer();
      })
    );

    // Listen for service errors
    unlisteners.push(
      listen('meril:service-error', (event) => {
        console.error('Meril service error:', event.payload);
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
  }, [fetchAnalyzer, toast]);

  // Initial fetch
  useEffect(() => {
    fetchAnalyzer();
  }, [fetchAnalyzer]);

  return {
    analyzer,
    loading,
    error,
    refreshAnalyzer,
    updateConfiguration,
    startService,
    stopService,
  };
} 