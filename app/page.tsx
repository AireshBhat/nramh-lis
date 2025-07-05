import { SystemStatus } from '@/components/dashboard/system-status';
import { DataFlowMonitor } from '@/components/dashboard/data-flow-monitor';
import { RecentEvents } from '@/components/dashboard/recent-events';

export default function Dashboard() {
  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold">LIS Dashboard</h1>
        <p className="text-muted-foreground">
          Real-time monitoring of laboratory data capture and HIS upload processes
        </p>
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