'use client';

import { LocalIpBanner } from '@/components/analyzers/local-ip-banner';
import { AnalyzerCard } from '@/components/analyzers/analyzer-card';
import { useLocalIp } from '@/hooks/use-local-ip';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';
import { useBF6500Analyzer } from '@/hooks/use-bf6500-analyzer';
import { Skeleton } from '@/components/ui/skeleton';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';

export default function AnalyzersPage() {
  const { localIp, isLoading: ipLoading } = useLocalIp();
  const { 
    analyzer: merilAnalyzer, 
    loading: merilLoading, 
    error: merilError, 
    refreshAnalyzer: refreshMerilAnalyzer, 
    startService: startMerilService, 
    stopService: stopMerilService 
  } = useMerilAnalyzer();
  
  const { 
    analyzer: bf6500Analyzer, 
    loading: bf6500Loading, 
    error: bf6500Error, 
    refreshAnalyzer: refreshBF6500Analyzer, 
    startService: startBF6500Service, 
    stopService: stopBF6500Service 
  } = useBF6500Analyzer();

  const handleMerilStatusChange = () => {
    refreshMerilAnalyzer();
  };

  const handleBF6500StatusChange = () => {
    refreshBF6500Analyzer();
  };

  const handleStartMerilService = async () => {
    try {
      await startMerilService();
    } catch (error) {
      console.error('Failed to start Meril service:', error);
    }
  };

  const handleStopMerilService = async () => {
    try {
      await stopMerilService();
    } catch (error) {
      console.error('Failed to stop Meril service:', error);
    }
  };

  const handleStartBF6500Service = async () => {
    try {
      await startBF6500Service();
    } catch (error) {
      console.error('Failed to start BF-6500 service:', error);
    }
  };

  const handleStopBF6500Service = async () => {
    try {
      await stopBF6500Service();
    } catch (error) {
      console.error('Failed to stop BF-6500 service:', error);
    }
  };

  const handleRefreshAll = async () => {
    await Promise.all([
      refreshMerilAnalyzer(),
      refreshBF6500Analyzer()
    ]);
  };

  const isLoading = merilLoading || bf6500Loading;
  const hasError = merilError || bf6500Error;
  const errorMessage = merilError || bf6500Error;

  return (
    <div className="p-4 md:p-6 space-y-6">
      {/* Local System IP Banner */}
      <LocalIpBanner localIp={localIp} isLoading={ipLoading} />

      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold">Analyzers</h1>
          <p className="text-muted-foreground">
            Monitor and manage the analyzers
          </p>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleRefreshAll}
            disabled={isLoading}
          >
            <RefreshCw className={`h-4 w-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
            Refresh All
          </Button>
        </div>
      </div>

      {/* Error Display */}
      {hasError && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{errorMessage}</AlertDescription>
        </Alert>
      )}

      {/* Loading State */}
      {isLoading && (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {[...Array(6)].map((_, i) => (
            <div key={i} className="space-y-4">
              <Skeleton className="h-48 w-full" />
            </div>
          ))}
        </div>
      )}

      {/* Analyzer Display */}
      {!isLoading && (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {/* Meril Analyzer */}
          {merilAnalyzer && (
            <AnalyzerCard 
              localIp={localIp}
              key={`meril-${merilAnalyzer.id}`} 
              analyzer={merilAnalyzer}
              onStatusChange={handleMerilStatusChange}
              onStart={handleStartMerilService}
              onStop={handleStopMerilService}
            />
          )}

          {/* BF-6500 Analyzer */}
          {bf6500Analyzer && (
            <AnalyzerCard 
              localIp={localIp}
              key={`bf6500-${bf6500Analyzer.id}`} 
              analyzer={bf6500Analyzer}
              onStatusChange={handleBF6500StatusChange}
              onStart={handleStartBF6500Service}
              onStop={handleStopBF6500Service}
            />
          )}

          {/* No Analyzers Found */}
          {!merilAnalyzer && !bf6500Analyzer && (
            <div className="col-span-full text-center py-12">
              <div className="text-muted-foreground">
                <AlertCircle className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <h3 className="text-lg font-medium mb-2">No analyzers found</h3>
                <p className="mb-4">No analyzer services are currently available.</p>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}