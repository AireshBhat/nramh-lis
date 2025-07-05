'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { 
  Activity, 
  Database, 
  Upload, 
  CheckCircle, 
  AlertCircle, 
  Clock,
  TrendingUp
} from 'lucide-react';
import { mockMetrics } from '@/lib/mock-data';

export function SystemStatus() {
  const uptime = mockMetrics.systemUptime;
  const successRate = (mockMetrics.successfulUploads / (mockMetrics.successfulUploads + mockMetrics.failedUploads)) * 100;

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            Samples Processed
          </CardTitle>
          <Database className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockMetrics.totalSamplesProcessed.toLocaleString()}</div>
          <p className="text-xs text-muted-foreground">
            {mockMetrics.samplesInQueue} in queue
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            Upload Success Rate
          </CardTitle>
          <TrendingUp className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{successRate.toFixed(1)}%</div>
          <div className="mt-2 w-full bg-secondary rounded-full h-4">
            <div 
              className="h-full bg-primary rounded-full transition-all" 
              style={{ width: `${successRate}%` }}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            Avg Processing Time
          </CardTitle>
          <Clock className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockMetrics.averageProcessingTime}s</div>
          <p className="text-xs text-muted-foreground">
            Per sample
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            System Uptime
          </CardTitle>
          <Activity className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{uptime}%</div>
          <div className="flex items-center space-x-2 mt-2">
            <div className="flex items-center space-x-1">
              <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
              <span className="text-xs text-muted-foreground">Online</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}