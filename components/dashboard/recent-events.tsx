'use client';

import { useState, useEffect, useMemo } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { 
  AlertCircle, 
  CheckCircle, 
  Info, 
  AlertTriangle,
  Clock
} from 'lucide-react';
import { SystemEvent } from '@/lib/types';
import { useTestResults } from '@/hooks/use-test-results';
import { useBF6900Analyzer } from '@/hooks/use-bf6900-analyzer';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';

export function RecentEvents() {
  const { testResults, statistics: testStats } = useTestResults({ autoLoad: true });
  const { serviceStatus: bf6900Status, error: bf6900Error } = useBF6900Analyzer();
  const { analyzer: merilAnalyzer, error: merilError } = useMerilAnalyzer();

  // Generate real events based on actual data
  const events = useMemo((): SystemEvent[] => {
    const realEvents: SystemEvent[] = [];

    // Add analyzer status events
    if (bf6900Status?.is_running) {
      realEvents.push({
        id: `BF6900-${Date.now()}`,
        timestamp: new Date(),
        type: 'SUCCESS',
        source: 'ANALYZER',
        message: 'BF-6900 analyzer service is running',
        details: { analyzer: 'BF-6900' }
      });
    }

    if (merilAnalyzer?.status?.status === 'Active') {
      realEvents.push({
        id: `MERIL-${Date.now()}`,
        timestamp: new Date(),
        type: 'SUCCESS',
        source: 'ANALYZER',
        message: 'Meril analyzer is active',
        details: { analyzer: 'Meril' }
      });
    }

    // Add test result events
    if (testStats.total > 0) {
      realEvents.push({
        id: `RESULTS-${Date.now()}`,
        timestamp: new Date(),
        type: 'INFO',
        source: 'DATABASE',
        message: `${testStats.total} total test results processed`,
        details: { 
          total: testStats.total,
          final: testStats.final,
          preliminary: testStats.preliminary,
          correction: testStats.correction
        }
      });
    }

    if (testStats.final > 0) {
      realEvents.push({
        id: `FINAL-${Date.now()}`,
        timestamp: new Date(),
        type: 'SUCCESS',
        source: 'UPLOAD',
        message: `${testStats.final} final results ready for upload`,
        details: { count: testStats.final }
      });
    }

    if (testStats.correction > 0) {
      realEvents.push({
        id: `CORRECTION-${Date.now()}`,
        timestamp: new Date(),
        type: 'WARNING',
        source: 'PROTOCOL',
        message: `${testStats.correction} results require correction`,
        details: { count: testStats.correction }
      });
    }

    if (testStats.abnormal > 0) {
      realEvents.push({
        id: `ABNORMAL-${Date.now()}`,
        timestamp: new Date(),
        type: 'WARNING',
        source: 'PROTOCOL',
        message: `${testStats.abnormal} abnormal results detected`,
        details: { count: testStats.abnormal }
      });
    }

    // Add error events
    if (bf6900Error) {
      realEvents.push({
        id: `BF6900-ERROR-${Date.now()}`,
        timestamp: new Date(),
        type: 'ERROR',
        source: 'ANALYZER',
        message: `BF-6900 analyzer error: ${bf6900Error}`,
        details: { error: bf6900Error }
      });
    }

    if (merilError) {
      realEvents.push({
        id: `MERIL-ERROR-${Date.now()}`,
        timestamp: new Date(),
        type: 'ERROR',
        source: 'ANALYZER',
        message: `Meril analyzer error: ${merilError}`,
        details: { error: merilError }
      });
    }

    // Add recent test result events
    const recentResults = testResults.slice(0, 5);
    recentResults.forEach((result, index) => {
      realEvents.push({
        id: `RESULT-${result.id}`,
        timestamp: result.completedDateTime || result.createdAt,
        type: result.status === 'Final' ? 'SUCCESS' : 
              result.status === 'Correction' ? 'WARNING' : 'INFO',
        source: 'PROTOCOL',
        message: `Test result ${result.testId} for sample ${result.sampleId} - ${result.status}`,
        details: { 
          testId: result.testId,
          sampleId: result.sampleId,
          status: result.status,
          value: result.value
        }
      });
    });

    // Sort by timestamp (newest first) and limit to 20 events
    return realEvents
      .sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
      .slice(0, 20);
  }, [testResults, testStats, bf6900Status, bf6900Error, merilAnalyzer, merilError]);

  const getEventIcon = (type: SystemEvent['type']) => {
    switch (type) {
      case 'SUCCESS':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'ERROR':
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      case 'WARNING':
        return <AlertTriangle className="h-4 w-4 text-yellow-500" />;
      default:
        return <Info className="h-4 w-4 text-blue-500" />;
    }
  };

  const getEventBadgeVariant = (type: SystemEvent['type']) => {
    switch (type) {
      case 'SUCCESS':
        return 'default';
      case 'ERROR':
        return 'destructive';
      case 'WARNING':
        return 'secondary';
      default:
        return 'outline';
    }
  };

  const getSourceColor = (source: SystemEvent['source']) => {
    switch (source) {
      case 'ANALYZER':
        return 'text-blue-600';
      case 'PROTOCOL':
        return 'text-purple-600';
      case 'DATABASE':
        return 'text-green-600';
      case 'UPLOAD':
        return 'text-orange-600';
      default:
        return 'text-gray-600';
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Clock className="h-5 w-5" />
          <span>Recent System Events</span>
        </CardTitle>
        <CardDescription>
          Real-time system events and notifications
        </CardDescription>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-80">
          <div className="space-y-3">
            {events.length > 0 ? (
              events.map((event) => (
                <div key={event.id} className="flex items-start space-x-3 p-3 rounded-lg border">
                  <div className="flex-shrink-0 mt-0.5">
                    {getEventIcon(event.type)}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center space-x-2 mb-1">
                      <Badge variant={getEventBadgeVariant(event.type)} className="text-xs">
                        {event.type}
                      </Badge>
                      <Badge variant="outline" className={`text-xs ${getSourceColor(event.source)}`}>
                        {event.source}
                      </Badge>
                    </div>
                    <div className="text-sm font-medium">{event.message}</div>
                    <div className="text-xs text-muted-foreground">
                      {event.timestamp.toLocaleTimeString()}
                    </div>
                  </div>
                </div>
              ))
            ) : (
              <div className="flex items-center justify-center h-32 text-muted-foreground">
                <div className="text-center">
                  <Info className="h-8 w-8 mx-auto mb-2" />
                  <p>No events to display</p>
                  <p className="text-xs">System events will appear here as they occur</p>
                </div>
              </div>
            )}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}