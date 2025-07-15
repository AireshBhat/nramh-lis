'use client';

import { SystemStatus } from '@/components/dashboard/system-status';
import { DataFlowMonitor } from '@/components/dashboard/data-flow-monitor';
import { RecentEvents } from '@/components/dashboard/recent-events';
import { useBF6900Analyzer } from '@/hooks/use-bf6900-analyzer';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';
import { Badge } from '@/components/ui/badge';
import { CheckCircle, AlertCircle } from 'lucide-react';

export default function Dashboard() {
  const { serviceStatus: bf6900Status } = useBF6900Analyzer();
  const { analyzer: merilAnalyzer } = useMerilAnalyzer();

  const analyzersOnline = [
    bf6900Status?.is_running,
    merilAnalyzer?.status?.status === 'Active'
  ].filter(Boolean).length;

  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold">LIS Dashboard</h1>
            <p className="text-muted-foreground">
              Real-time monitoring of laboratory data capture and HIS upload processes
            </p>
          </div>
          <div className="flex items-center space-x-2">
            <Badge variant={analyzersOnline > 0 ? "default" : "secondary"} className="flex items-center space-x-1">
              {analyzersOnline > 0 ? (
                <CheckCircle className="h-3 w-3" />
              ) : (
                <AlertCircle className="h-3 w-3" />
              )}
              <span>{analyzersOnline}/2 Analyzers Online</span>
            </Badge>
          </div>
        </div>
      </div>

      <SystemStatus />
      
      <div className="grid gap-6 lg:grid-cols-3">
        <div className="lg:col-span-2">
          <DataFlowMonitor />
        </div>
        <div className="lg:col-span-1">
          <RecentEvents />
        </div>
      </div>
    </div>
  );
}