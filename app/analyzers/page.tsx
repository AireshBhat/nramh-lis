'use client';

import { useState } from 'react';
import { LocalIpBanner } from '@/components/analyzers/local-ip-banner';
import { AnalyzerCard } from '@/components/analyzers/analyzer-card';
import { AnalyzerConfigDialog } from '@/components/analyzers/analyzer-config-dialog';
import { useLocalIp } from '@/hooks/use-local-ip';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';
import { useBF6900Analyzer } from '@/hooks/use-bf6900-analyzer';
import { Skeleton } from '@/components/ui/skeleton';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Analyzer } from '@/lib/types';

export default function AnalyzersPage() {
  const { localIp, isLoading: ipLoading } = useLocalIp();
  const [configDialogOpen, setConfigDialogOpen] = useState(false);
  const [selectedAnalyzer, setSelectedAnalyzer] = useState<Analyzer | null>(null);
  const [configUpdateFunction, setConfigUpdateFunction] = useState<((updatedAnalyzer: Partial<Analyzer>) => Promise<void>) | null>(null);
  
  const { 
    analyzer: merilAnalyzer, 
    loading: merilLoading, 
    error: merilError, 
    refreshAnalyzer: refreshMerilAnalyzer, 
    updateConfiguration: updateMerilConfiguration,
    startService: startMerilService, 
    stopService: stopMerilService 
  } = useMerilAnalyzer();
  
  const { 
    analyzer: bf6900Analyzer, 
    loading: bf6900Loading, 
    error: bf6900Error, 
    refreshAnalyzer: refreshBF6900Analyzer, 
    updateConfiguration: updateBF6900Configuration,
    startService: startBF6900Service, 
    stopService: stopBF6900Service 
  } = useBF6900Analyzer();

  const handleMerilStatusChange = () => {
    refreshMerilAnalyzer();
  };

  const handleBF6900StatusChange = () => {
    refreshBF6900Analyzer();
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

  const handleStartBF6900Service = async () => {
    try {
      await startBF6900Service();
    } catch (error) {
      console.error('Failed to start BF-6900 service:', error);
    }
  };

  const handleStopBF6900Service = async () => {
    try {
      await stopBF6900Service();
    } catch (error) {
      console.error('Failed to stop BF-6900 service:', error);
    }
  };

  const handleRefreshAll = async () => {
    await Promise.all([
      refreshMerilAnalyzer(),
      refreshBF6900Analyzer()
    ]);
  };

  const handleConfigureMeril = () => {
    if (merilAnalyzer) {
      setSelectedAnalyzer(merilAnalyzer);
      setConfigUpdateFunction(() => updateMerilConfiguration);
      setConfigDialogOpen(true);
    }
  };

  const handleConfigureBF6900 = () => {
    if (bf6900Analyzer) {
      setSelectedAnalyzer(bf6900Analyzer);
      setConfigUpdateFunction(() => updateBF6900Configuration);
      setConfigDialogOpen(true);
    }
  };

  const handleSaveConfiguration = async (updatedAnalyzer: Partial<Analyzer>) => {
    if (configUpdateFunction) {
      await configUpdateFunction(updatedAnalyzer);
    }
  };

  const isLoading = merilLoading || bf6900Loading;
  const hasError = merilError || bf6900Error;
  const errorMessage = merilError || bf6900Error;

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
              onConfigure={handleConfigureMeril}
              onRefresh={refreshMerilAnalyzer}
              refreshing={merilLoading}
            />
          )}

          {/* BF-6900 Analyzer */}
          {bf6900Analyzer && (
            <AnalyzerCard 
              localIp={localIp}
              key={`bf6900-${bf6900Analyzer.id}`} 
              analyzer={bf6900Analyzer}
              onStatusChange={handleBF6900StatusChange}
              onStart={handleStartBF6900Service}
              onStop={handleStopBF6900Service}
              onConfigure={handleConfigureBF6900}
              onRefresh={refreshBF6900Analyzer}
              refreshing={bf6900Loading}
            />
          )}

          {/* No Analyzers Found */}
          {!merilAnalyzer && !bf6900Analyzer && (
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

      {/* Configuration Dialog */}
      <AnalyzerConfigDialog
        analyzer={selectedAnalyzer}
        open={configDialogOpen}
        onOpenChange={setConfigDialogOpen}
        onSave={handleSaveConfiguration}
        loading={merilLoading || bf6900Loading}
      />
    </div>
  );
}