'use client';

import { useState, useEffect } from 'react';
import { useRouter, useParams } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ArrowLeft, Play, Square, RefreshCw, Activity, AlertCircle } from 'lucide-react';
import { Analyzer } from '@/lib/types';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';
import { useBF6900Analyzer } from '@/hooks/use-bf6900-analyzer';
import { AnalyzerDetailsCard } from '@/components/analyzers/analyzer-details-card';
import { LabResultsPollingDashboard } from '@/components/analyzers/lab-results-polling-dashboard';
import { Skeleton } from '@/components/ui/skeleton';
import { Alert, AlertDescription } from '@/components/ui/alert';

export default function AnalyzerDetailPage() {
  const router = useRouter();
  const params = useParams();
  const analyzerId = params.id as string;
  
  const [analyzer, setAnalyzer] = useState<Analyzer | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Get analyzer data from appropriate hook based on ID
  const { 
    analyzer: merilAnalyzer, 
    loading: merilLoading, 
    error: merilError,
    refreshAnalyzer: refreshMerilAnalyzer,
    startService: startMerilService,
    stopService: stopMerilService
  } = useMerilAnalyzer();
  
  const { 
    analyzer: bf6900Analyzer, 
    loading: bf6900Loading, 
    error: bf6900Error,
    refreshAnalyzer: refreshBF6900Analyzer,
    startService: startBF6900Service,
    stopService: stopBF6900Service
  } = useBF6900Analyzer();

  // Determine which analyzer we're viewing and set up appropriate data
  useEffect(() => {
    const determineAnalyzer = async () => {
      try {
        setLoading(true);
        setError(null);

        // Check if this is the Meril analyzer
        if (merilAnalyzer && (merilAnalyzer.id === analyzerId || analyzerId === 'meril' || analyzerId === 'autoquant')) {
          setAnalyzer(merilAnalyzer);
          setLoading(merilLoading);
          setError(merilError);
          return;
        }

        // Check if this is the BF-6900 analyzer
        if (bf6900Analyzer && (bf6900Analyzer.id === analyzerId || analyzerId === 'bf6900' || analyzerId === 'cq5plus')) {
          setAnalyzer(bf6900Analyzer);
          setLoading(bf6900Loading);
          setError(bf6900Error);
          return;
        }

        // If no analyzer found, show error
        if (!merilLoading && !bf6900Loading) {
          setError('Analyzer not found');
          setLoading(false);
        }
      } catch (err) {
        setError('Failed to load analyzer details');
        setLoading(false);
      }
    };

    determineAnalyzer();
  }, [analyzerId, merilAnalyzer, bf6900Analyzer, merilLoading, bf6900Loading, merilError, bf6900Error]);

  const handleBack = () => {
    router.push('/analyzers');
  };

  const handleRefresh = async () => {
    if (analyzer?.id === merilAnalyzer?.id) {
      await refreshMerilAnalyzer();
    } else if (analyzer?.id === bf6900Analyzer?.id) {
      await refreshBF6900Analyzer();
    }
  };

  const handleServiceToggle = async () => {
    try {
      if (!analyzer) return;

      if (analyzer.status.status === 'Active') {
        // Stop service
        if (analyzer.id === merilAnalyzer?.id) {
          await stopMerilService();
        } else if (analyzer.id === bf6900Analyzer?.id) {
          await stopBF6900Service();
        }
      } else {
        // Start service
        if (analyzer.id === merilAnalyzer?.id) {
          await startMerilService();
        } else if (analyzer.id === bf6900Analyzer?.id) {
          await startBF6900Service();
        }
      }
      
      // Refresh analyzer status
      await handleRefresh();
    } catch (error) {
      console.error('Failed to toggle service:', error);
    }
  };

  const getAnalyzerTypeName = (analyzer: Analyzer | null): string => {
    if (!analyzer) return 'Unknown';
    if (analyzer.protocol.protocol === 'Astm') return 'AutoQuant';
    if (analyzer.protocol.protocol.includes('HL7')) return 'Meril CQ 5 Plus';
    return analyzer.name;
  };

  if (loading) {
    return (
      <div className="p-4 md:p-6 space-y-6">
        {/* Header Skeleton */}
        <div className="flex items-center space-x-4">
          <Skeleton className="h-10 w-24" />
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-10 w-20" />
        </div>
        
        {/* Content Skeleton */}
        <div className="grid gap-6 md:grid-cols-2">
          <Skeleton className="h-64 w-full" />
          <Skeleton className="h-64 w-full" />
        </div>
        <Skeleton className="h-96 w-full" />
      </div>
    );
  }

  if (error || !analyzer) {
    return (
      <div className="p-4 md:p-6 space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <Button 
              variant="outline" 
              size="sm" 
              onClick={handleBack}
              className="flex items-center space-x-2"
            >
              <ArrowLeft className="h-4 w-4" />
              <span>Back</span>
            </Button>
            <h1 className="text-3xl font-bold">Analyzer Details</h1>
          </div>
        </div>

        {/* Error Display */}
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>
            {error || 'Analyzer not found. Please check if the analyzer exists and try again.'}
          </AlertDescription>
        </Alert>

        <div className="text-center py-12">
          <Button onClick={handleBack} variant="outline">
            Return to Analyzers
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 md:p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <Button 
            variant="outline" 
            size="sm" 
            onClick={handleBack}
            className="flex items-center space-x-2"
          >
            <ArrowLeft className="h-4 w-4" />
            <span>Back</span>
          </Button>
          
          <div>
            <h1 className="text-3xl font-bold flex items-center space-x-2">
              <Activity className="h-8 w-8 text-primary" />
              <span>{getAnalyzerTypeName(analyzer)}</span>
              <Badge 
                variant={analyzer.status.status === 'Active' ? 'default' : 'secondary'}
                className={analyzer.status.status === 'Active' ? 'bg-green-100 text-green-800' : ''}
              >
                {analyzer.status.status}
              </Badge>
            </h1>
            <p className="text-muted-foreground">
              {analyzer.manufacturer} {analyzer.model}
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleRefresh}
            disabled={loading}
          >
            <RefreshCw className={`h-4 w-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          
          <Button
            onClick={handleServiceToggle}
            variant={analyzer.status.status === 'Active' ? 'destructive' : 'default'}
            size="sm"
          >
            {analyzer.status.status === 'Active' ? (
              <>
                <Square className="h-4 w-4 mr-2" />
                Stop Service
              </>
            ) : (
              <>
                <Play className="h-4 w-4 mr-2" />
                Start Service
              </>
            )}
          </Button>
        </div>
      </div>

      {/* Main Content */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Analyzer Details */}
        <div className="lg:col-span-1">
          <AnalyzerDetailsCard analyzer={analyzer} />
        </div>

        {/* Lab Results Dashboard */}
        <div className="lg:col-span-2">
          <LabResultsPollingDashboard 
            analyzer={analyzer}
            isServiceActive={analyzer.status.status === 'Active'}
          />
        </div>
      </div>

      {/* Real-time Results Feed */}
      <Card>
        <CardHeader>
          <CardTitle>Real-time Lab Results</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground">
            <Activity className="h-12 w-12 mx-auto mb-4 opacity-50" />
            <p>Lab results will appear here when the analyzer is active and polling is enabled.</p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}