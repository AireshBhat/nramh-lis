'use client';

import { LocalIpBanner } from '@/components/analyzers/local-ip-banner';
import { AnalyzerCard } from '@/components/analyzers/analyzer-card';
import { useLocalIp } from '@/hooks/use-local-ip';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';
import { Skeleton } from '@/components/ui/skeleton';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';

export default function AnalyzersPage() {
  const { localIp, isLoading: ipLoading } = useLocalIp();
  const { analyzer, loading, error, refreshAnalyzer, startService } = useMerilAnalyzer();

  const handleStatusChange = () => {
    refreshAnalyzer();
  };

  const handleStartService = async () => {
    try {
      await startService();
    } catch (error) {
      console.error('Failed to start service:', error);
    }
  };

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
            onClick={refreshAnalyzer}
            disabled={loading}
          >
            <RefreshCw className={`h-4 w-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
        </div>
      </div>

      {/* Error Display */}
      {error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Loading State */}
      {loading && (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {[...Array(6)].map((_, i) => (
            <div key={i} className="space-y-4">
              <Skeleton className="h-48 w-full" />
            </div>
          ))}
        </div>
      )}

      {/* Analyzer Display */}
      {!loading && (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {!analyzer ? (
            <div className="col-span-full text-center py-12">
              <div className="text-muted-foreground">
                <AlertCircle className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <h3 className="text-lg font-medium mb-2">No Meril analyzer found</h3>
                <p className="mb-4">The Meril AutoQuant analyzer service is not available.</p>
              </div>
            </div>
          ) : (
            <AnalyzerCard 
              key={analyzer.id} 
              analyzer={analyzer}
              onStatusChange={handleStatusChange}
              onStart={handleStartService}
            />
          )}
        </div>
      )}
    </div>
  );
}