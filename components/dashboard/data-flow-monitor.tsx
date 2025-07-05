'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { 
  ArrowRight, 
  Database, 
  Upload, 
  CheckCircle, 
  AlertCircle, 
  Clock,
  RefreshCw,
  Activity
} from 'lucide-react';

interface FlowStage {
  id: string;
  name: string;
  status: 'active' | 'completed' | 'error' | 'idle';
  count: number;
  description: string;
}

export function DataFlowMonitor() {
  const [captureFlow, setCaptureFlow] = useState<FlowStage[]>([
    { id: 'receive', name: 'Data Reception', status: 'active', count: 3, description: 'Receiving data from analyzers' },
    { id: 'parse', name: 'Protocol Parsing', status: 'active', count: 2, description: 'ASTM/HL7 message parsing' },
    { id: 'validate', name: 'Data Validation', status: 'completed', count: 1, description: 'Validating test results' },
    { id: 'store', name: 'Database Storage', status: 'completed', count: 1, description: 'Storing in local database' }
  ]);

  const [uploadFlow, setUploadFlow] = useState<FlowStage[]>([
    { id: 'queue', name: 'Upload Queue', status: 'active', count: 5, description: 'Queued for HIS upload' },
    { id: 'format', name: 'Data Formatting', status: 'active', count: 2, description: 'Formatting for HIS system' },
    { id: 'upload', name: 'HIS Upload', status: 'active', count: 1, description: 'Uploading to HIS system' },
    { id: 'confirm', name: 'Confirmation', status: 'error', count: 1, description: 'Awaiting HIS confirmation' }
  ]);

  const [lastUpdate, setLastUpdate] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => {
      // Simulate real-time updates
      setCaptureFlow(prev => prev.map(stage => ({
        ...stage,
        count: Math.max(0, stage.count + (Math.random() > 0.5 ? 1 : -1))
      })));
      
      setUploadFlow(prev => prev.map(stage => ({
        ...stage,
        count: Math.max(0, stage.count + (Math.random() > 0.5 ? 1 : -1))
      })));
      
      setLastUpdate(new Date());
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  const getStatusIcon = (status: FlowStage['status']) => {
    switch (status) {
      case 'active':
        return <Activity className="h-4 w-4 text-blue-500 animate-pulse" />;
      case 'completed':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'error':
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      default:
        return <Clock className="h-4 w-4 text-gray-500" />;
    }
  };

  const getStatusColor = (status: FlowStage['status']) => {
    switch (status) {
      case 'active':
        return 'bg-blue-500';
      case 'completed':
        return 'bg-green-500';
      case 'error':
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  };

  const FlowVisualization = ({ 
    title, 
    description, 
    icon: Icon, 
    stages 
  }: { 
    title: string;
    description: string;
    icon: React.ElementType;
    stages: FlowStage[];
  }) => (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Icon className="h-5 w-5" />
          <span>{title}</span>
        </CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {stages.map((stage, index) => (
            <div key={stage.id} className="relative">
              <div className="flex items-center justify-between p-3 rounded-lg border">
                <div className="flex items-center space-x-3">
                  {getStatusIcon(stage.status)}
                  <div>
                    <div className="font-medium">{stage.name}</div>
                    <div className="text-sm text-muted-foreground">{stage.description}</div>
                  </div>
                </div>
                <Badge variant="secondary" className="flex items-center space-x-1">
                  <span>{stage.count}</span>
                </Badge>
              </div>
              {index < stages.length - 1 && (
                <div className="flex justify-center my-2">
                  <ArrowRight className="h-4 w-4 text-muted-foreground" />
                </div>
              )}
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">Data Flow Monitoring</h2>
        <div className="flex items-center space-x-2">
          <span className="text-sm text-muted-foreground">
            Last updated: {lastUpdate.toLocaleTimeString()}
          </span>
          <Button variant="outline" size="sm">
            <RefreshCw className="h-4 w-4 mr-2" />
            Refresh
          </Button>
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        <FlowVisualization
          title="Laboratory Data Capture"
          description="Real-time monitoring of data capture from laboratory analyzers"
          icon={Database}
          stages={captureFlow}
        />

        <FlowVisualization
          title="HIS Upload Process"
          description="Monitoring upload process to Hospital Information System"
          icon={Upload}
          stages={uploadFlow}
        />
      </div>
    </div>
  );
}