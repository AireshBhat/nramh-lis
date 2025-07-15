'use client';

import { useState, useEffect, useMemo } from 'react';
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
import { useTestResults } from '@/hooks/use-test-results';
import { useBF6900Analyzer } from '@/hooks/use-bf6900-analyzer';
import { useMerilAnalyzer } from '@/hooks/use-meril-analyzer';

interface FlowStage {
  id: string;
  name: string;
  status: 'active' | 'completed' | 'error' | 'idle';
  count: number;
  description: string;
}

export function DataFlowMonitor() {
  const { statistics: testStats, testResults } = useTestResults({ autoLoad: true });
  const { serviceStatus: bf6900Status } = useBF6900Analyzer();
  const { analyzer: merilAnalyzer } = useMerilAnalyzer();
  const [lastUpdate, setLastUpdate] = useState(new Date());

  // Calculate flow stages based on real data
  const captureFlow = useMemo((): FlowStage[] => {
    const totalResults = testStats.total;
    const preliminaryResults = testStats.preliminary;
    const finalResults = testStats.final;
    const abnormalResults = testStats.abnormal;

    return [
      { 
        id: 'receive', 
        name: 'Data Reception', 
        status: bf6900Status?.is_running || merilAnalyzer?.status?.status === 'Active' ? 'active' : 'idle', 
        count: preliminaryResults, 
        description: 'Receiving data from analyzers' 
      },
      { 
        id: 'parse', 
        name: 'Protocol Parsing', 
        status: preliminaryResults > 0 ? 'active' : 'idle', 
        count: preliminaryResults, 
        description: 'ASTM/HL7 message parsing' 
      },
      { 
        id: 'validate', 
        name: 'Data Validation', 
        status: finalResults > 0 ? 'completed' : 'idle', 
        count: finalResults, 
        description: 'Validating test results' 
      },
      { 
        id: 'store', 
        name: 'Database Storage', 
        status: totalResults > 0 ? 'completed' : 'idle', 
        count: totalResults, 
        description: 'Storing in local database' 
      }
    ];
  }, [testStats, bf6900Status, merilAnalyzer]);

  const uploadFlow = useMemo((): FlowStage[] => {
    const finalResults = testStats.final;
    const correctionResults = testStats.correction;
    const totalUploads = finalResults + correctionResults;

    return [
      { 
        id: 'queue', 
        name: 'Upload Queue', 
        status: finalResults > 0 ? 'active' : 'idle', 
        count: finalResults, 
        description: 'Queued for HIS upload' 
      },
      { 
        id: 'format', 
        name: 'Data Formatting', 
        status: finalResults > 0 ? 'active' : 'idle', 
        count: finalResults, 
        description: 'Formatting for HIS system' 
      },
      { 
        id: 'upload', 
        name: 'HIS Upload', 
        status: finalResults > 0 ? 'active' : 'idle', 
        count: finalResults, 
        description: 'Uploading to HIS system' 
      },
      { 
        id: 'confirm', 
        name: 'Confirmation', 
        status: correctionResults > 0 ? 'error' : (finalResults > 0 ? 'completed' : 'idle'), 
        count: correctionResults, 
        description: 'Awaiting HIS confirmation' 
      }
    ];
  }, [testStats]);

  const refreshData = () => {
    setLastUpdate(new Date());
  };

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
          <Button variant="outline" size="sm" onClick={refreshData}>
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