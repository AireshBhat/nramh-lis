'use client';

import { useState, useEffect, useCallback } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';
import { Switch } from '@/components/ui/switch';
import { 
  Play, 
  Square, 
  RefreshCw, 
  Clock, 
  Activity,
  AlertTriangle,
  CheckCircle2,
  Timer,
  Zap,
  Database
} from 'lucide-react';
import { Analyzer } from '@/lib/types';
import { Alert, AlertDescription } from '@/components/ui/alert';

interface LabResultsPollingDashboardProps {
  analyzer: Analyzer;
  isServiceActive: boolean;
}

interface PollingStatus {
  isPolling: boolean;
  interval: number;
  lastPoll: Date | null;
  nextPoll: Date | null;
  successfulPolls: number;
  failedPolls: number;
}

export function LabResultsPollingDashboard({ analyzer, isServiceActive }: LabResultsPollingDashboardProps) {
  const [pollingStatus, setPollingStatus] = useState<PollingStatus>({
    isPolling: false,
    interval: 10000, // 10 seconds default
    lastPoll: null,
    nextPoll: null,
    successfulPolls: 0,
    failedPolls: 0
  });
  
  const [pollingInterval, setPollingInterval] = useState(10); // seconds
  const [autoStart, setAutoStart] = useState(false);
  const [pollingTimer, setPollingTimer] = useState<NodeJS.Timeout | null>(null);

  // Calculate next poll countdown
  const [countdown, setCountdown] = useState<number | null>(null);

  // Update countdown every second
  useEffect(() => {
    if (!pollingStatus.isPolling || !pollingStatus.nextPoll) {
      setCountdown(null);
      return;
    }

    const interval = setInterval(() => {
      const now = new Date();
      const remaining = Math.max(0, Math.floor((pollingStatus.nextPoll!.getTime() - now.getTime()) / 1000));
      setCountdown(remaining);

      if (remaining === 0) {
        // Trigger next poll
        performPoll();
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [pollingStatus.isPolling, pollingStatus.nextPoll]);

  // Mock polling function - this would integrate with actual HL7 polling
  const performPoll = useCallback(async () => {
    try {
      console.log(`Polling ${analyzer.name} for lab results...`);
      
      // TODO: Implement actual HL7 QRY^A19^QRY_Q02 query here
      // This would send a query message to the lab instrument and await DSR response
      
      // For now, simulate polling behavior
      const success = Math.random() > 0.1; // 90% success rate for demo
      
      const now = new Date();
      const nextPollTime = new Date(now.getTime() + pollingStatus.interval);
      
      setPollingStatus(prev => ({
        ...prev,
        lastPoll: now,
        nextPoll: nextPollTime,
        successfulPolls: success ? prev.successfulPolls + 1 : prev.successfulPolls,
        failedPolls: success ? prev.failedPolls : prev.failedPolls + 1
      }));

      console.log(`Poll ${success ? 'successful' : 'failed'} at ${now.toLocaleTimeString()}`);
      
    } catch (error) {
      console.error('Polling error:', error);
      setPollingStatus(prev => ({
        ...prev,
        lastPoll: new Date(),
        failedPolls: prev.failedPolls + 1
      }));
    }
  }, [analyzer.name, pollingStatus.interval]);

  const startPolling = useCallback(() => {
    if (!isServiceActive) {
      console.warn('Cannot start polling: Service is not active');
      return;
    }

    console.log(`Starting polling every ${pollingInterval} seconds`);
    
    const intervalMs = pollingInterval * 1000;
    const now = new Date();
    const nextPollTime = new Date(now.getTime() + intervalMs);
    
    setPollingStatus(prev => ({
      ...prev,
      isPolling: true,
      interval: intervalMs,
      nextPoll: nextPollTime
    }));

    // Perform initial poll immediately
    performPoll();
  }, [isServiceActive, pollingInterval, performPoll]);

  const stopPolling = useCallback(() => {
    console.log('Stopping polling');
    
    if (pollingTimer) {
      clearInterval(pollingTimer);
      setPollingTimer(null);
    }

    setPollingStatus(prev => ({
      ...prev,
      isPolling: false,
      nextPoll: null
    }));
    
    setCountdown(null);
  }, [pollingTimer]);

  const resetStats = () => {
    setPollingStatus(prev => ({
      ...prev,
      successfulPolls: 0,
      failedPolls: 0,
      lastPoll: null
    }));
  };

  // Auto-start polling when service becomes active
  useEffect(() => {
    if (autoStart && isServiceActive && !pollingStatus.isPolling) {
      startPolling();
    }
    if (!isServiceActive && pollingStatus.isPolling) {
      stopPolling();
    }
  }, [autoStart, isServiceActive, pollingStatus.isPolling, startPolling, stopPolling]);

  const formatTime = (date: Date | null) => {
    if (!date) return 'Never';
    return date.toLocaleTimeString();
  };

  const getPollingStatusBadge = () => {
    if (!isServiceActive) {
      return <Badge variant="destructive">Service Inactive</Badge>;
    }
    if (pollingStatus.isPolling) {
      return <Badge className="bg-green-100 text-green-800">Polling Active</Badge>;
    }
    return <Badge variant="outline">Polling Stopped</Badge>;
  };

  const getProtocolInfo = () => {
    if (analyzer.protocol.protocol === 'Astm') {
      return {
        name: 'ASTM E1394-97',
        queryType: 'Request Record (Q)',
        description: 'Uses ASTM query protocol to request lab results'
      };
    } else if (analyzer.protocol.protocol.includes('HL7')) {
      return {
        name: 'HL7 v2.3.1',
        queryType: 'QRY^A19^QRY_Q02',
        description: 'Uses HL7 query messages to request lab results from external system'
      };
    }
    return {
      name: 'Unknown Protocol',
      queryType: 'N/A',
      description: 'Protocol not supported for polling'
    };
  };

  const protocolInfo = getProtocolInfo();

  return (
    <div className="space-y-6">
      {/* Polling Control Card */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Activity className="h-5 w-5" />
            <span>Lab Results Polling</span>
            {getPollingStatusBadge()}
          </CardTitle>
          <CardDescription>
            Automatically query the lab instrument for new test results using {protocolInfo.name}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Service Status Warning */}
          {!isServiceActive && (
            <Alert>
              <AlertTriangle className="h-4 w-4" />
              <AlertDescription>
                Analyzer service must be active to enable polling. Please start the service first.
              </AlertDescription>
            </Alert>
          )}

          {/* Protocol Information */}
          <div className="bg-muted/50 p-4 rounded-lg space-y-2">
            <h4 className="text-sm font-medium flex items-center space-x-2">
              <Database className="h-4 w-4" />
              <span>Protocol Details</span>
            </h4>
            <div className="text-sm text-muted-foreground space-y-1">
              <p><strong>Protocol:</strong> {protocolInfo.name}</p>
              <p><strong>Query Type:</strong> {protocolInfo.queryType}</p>
              <p><strong>Description:</strong> {protocolInfo.description}</p>
            </div>
          </div>

          {/* Polling Configuration */}
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="space-y-1">
                <Label htmlFor="polling-interval">Polling Interval (seconds)</Label>
                <p className="text-sm text-muted-foreground">How often to query for new results</p>
              </div>
              <Input
                id="polling-interval"
                type="number"
                min="5"
                max="300"
                value={pollingInterval}
                onChange={(e) => setPollingInterval(parseInt(e.target.value) || 10)}
                className="w-20"
                disabled={pollingStatus.isPolling}
              />
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-1">
                <Label htmlFor="auto-start">Auto-start with Service</Label>
                <p className="text-sm text-muted-foreground">Start polling when analyzer service becomes active</p>
              </div>
              <Switch
                id="auto-start"
                checked={autoStart}
                onCheckedChange={setAutoStart}
              />
            </div>
          </div>

          <Separator />

          {/* Polling Controls */}
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              {pollingStatus.isPolling ? (
                <Button onClick={stopPolling} variant="destructive" size="sm">
                  <Square className="h-4 w-4 mr-2" />
                  Stop Polling
                </Button>
              ) : (
                <Button 
                  onClick={startPolling} 
                  disabled={!isServiceActive}
                  size="sm"
                >
                  <Play className="h-4 w-4 mr-2" />
                  Start Polling
                </Button>
              )}
              
              <Button onClick={resetStats} variant="outline" size="sm">
                <RefreshCw className="h-4 w-4 mr-2" />
                Reset Stats
              </Button>
            </div>

            {countdown !== null && (
              <div className="flex items-center space-x-2 text-sm text-muted-foreground">
                <Timer className="h-4 w-4" />
                <span>Next poll in {countdown}s</span>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Polling Statistics */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Zap className="h-5 w-5" />
            <span>Polling Statistics</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="space-y-2">
              <div className="flex items-center space-x-2">
                <CheckCircle2 className="h-4 w-4 text-green-500" />
                <Label className="text-sm text-muted-foreground">Successful</Label>
              </div>
              <p className="text-2xl font-bold text-green-600">{pollingStatus.successfulPolls}</p>
            </div>

            <div className="space-y-2">
              <div className="flex items-center space-x-2">
                <AlertTriangle className="h-4 w-4 text-red-500" />
                <Label className="text-sm text-muted-foreground">Failed</Label>
              </div>
              <p className="text-2xl font-bold text-red-600">{pollingStatus.failedPolls}</p>
            </div>

            <div className="space-y-2">
              <div className="flex items-center space-x-2">
                <Clock className="h-4 w-4 text-blue-500" />
                <Label className="text-sm text-muted-foreground">Last Poll</Label>
              </div>
              <p className="text-sm font-mono">{formatTime(pollingStatus.lastPoll)}</p>
            </div>

            <div className="space-y-2">
              <div className="flex items-center space-x-2">
                <Timer className="h-4 w-4 text-orange-500" />
                <Label className="text-sm text-muted-foreground">Interval</Label>
              </div>
              <p className="text-sm font-mono">{pollingStatus.interval / 1000}s</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}