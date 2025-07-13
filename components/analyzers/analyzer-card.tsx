import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Monitor, Wifi, AlertCircle, CheckCircle, Wrench, Play } from 'lucide-react';
import { Analyzer } from '@/lib/types';

interface AnalyzerCardProps {
  analyzer: Analyzer;
  onStatusChange?: () => void;
  onStart?: () => Promise<void>;
}

export function AnalyzerCard({ analyzer, onStatusChange, onStart }: AnalyzerCardProps) {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Active':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'Maintenance':
        return <Wrench className="h-4 w-4 text-yellow-500" />;
      default:
        return <AlertCircle className="h-4 w-4 text-red-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Active':
        return 'bg-green-100 text-green-800';
      case 'Maintenance':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-red-100 text-red-800';
    }
  };

  const handleStart = async () => {
    if (onStart) {
      try {
        await onStart();
        if (onStatusChange) {
          onStatusChange();
        }
      } catch (error) {
        console.error('Failed to start analyzer:', error);
      }
    }
  };

  return (
    <Card className="relative">
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Monitor className="h-5 w-5" />
          <span>{analyzer.name}</span>
        </CardTitle>
        <CardDescription>
          {analyzer.manufacturer} {analyzer.model}
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Status</span>
          <div className="flex items-center space-x-2">
            {getStatusIcon(analyzer.status.status)}
            <Badge className={getStatusColor(analyzer.status.status)}>
              {analyzer.status.status}
            </Badge>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Connection</span>
          <div className="flex items-center space-x-2">
            <Wifi className="h-4 w-4 text-muted-foreground" />
            <span className="text-sm">{analyzer.connectionType.type}</span>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Protocol</span>
          <Badge variant="outline">{analyzer.protocol.protocol}</Badge>
        </div>

        {analyzer.serialNumber && (
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Serial Number</span>
            <span className="text-sm font-mono">{analyzer.serialNumber}</span>
          </div>
        )}

        {analyzer.ipAddress && (
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">IP Address</span>
            <span className="text-sm font-mono">{analyzer.ipAddress}:{analyzer.port}</span>
          </div>
        )}

        {onStart && analyzer.status.status !== 'Active' && (
          <div className="pt-2">
            <Button 
              onClick={handleStart}
              className="w-full"
              size="sm"
            >
              <Play className="h-4 w-4 mr-2" />
              Start Service
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  );
} 