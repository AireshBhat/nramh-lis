import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Monitor, Wifi, AlertCircle, CheckCircle, Wrench, Play, Square, Settings, RefreshCw, ExternalLink } from 'lucide-react';
import { Analyzer } from '@/lib/types';
import { useRouter } from 'next/navigation';

interface AnalyzerCardProps {
  analyzer: Analyzer;
  localIp: string;
  onStatusChange?: () => void;
  onStart?: () => Promise<void>;
  onStop?: () => Promise<void>;
  onConfigure?: () => void;
  onRefresh?: () => Promise<void>;
  refreshing?: boolean;
}

export function AnalyzerCard({ analyzer, localIp, onStatusChange, onStart, onStop, onConfigure, onRefresh, refreshing }: AnalyzerCardProps) {
  const router = useRouter();
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

  const handleStop = async () => {
    if (onStop) {
      try {
        await onStop();
        if (onStatusChange) {
          onStatusChange();
        }
      } catch (error) {
        console.error('Failed to stop analyzer:', error);
      }
    }
  };

  const handleRefresh = async () => {
    if (onRefresh) {
      try {
        await onRefresh();
        if (onStatusChange) {
          onStatusChange();
        }
      } catch (error) {
        console.error('Failed to refresh analyzer:', error);
      }
    }
  };

  const getAnalyzerRoute = () => {
    // Use analyzer type to determine route
    if (analyzer.protocol.protocol === 'Astm') {
      return '/analyzers/meril';
    } else if (analyzer.protocol.protocol.includes('HL7')) {
      return '/analyzers/bf6900';
    }
    return `/analyzers/${analyzer.id}`;
  };

  const handleViewDetails = () => {
    router.push(getAnalyzerRoute());
  };

  return (
    <Card className="relative flex flex-col h-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div 
            className="flex items-center space-x-2 cursor-pointer hover:text-primary transition-colors"
            onClick={handleViewDetails}
          >
            <Monitor className="h-5 w-5" />
            <CardTitle className="hover:underline">{analyzer.name}</CardTitle>
            <ExternalLink className="h-4 w-4 opacity-50" />
          </div>
          {onRefresh && (
            <Button
              variant="ghost"
              size="sm"
              onClick={handleRefresh}
              disabled={refreshing}
              className="h-8 w-8 p-0"
            >
              <RefreshCw className={`h-4 w-4 ${refreshing ? 'animate-spin' : ''}`} />
            </Button>
          )}
        </div>
        <CardDescription>
          {analyzer.manufacturer} {analyzer.model}
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col flex-1">
        {/* Analyzer Information */}
        <div className="space-y-4">
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

          {analyzer.port && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Local Listener</span>
              <span className="text-sm font-mono">{localIp}:{analyzer.port}</span>
            </div>
          )}

          {analyzer.external_ip && analyzer.external_port && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">External LIS</span>
              <span className="text-sm font-mono">{analyzer.external_ip}:{analyzer.external_port}</span>
            </div>
          )}
        </div>

        {/* Spacer to push buttons to bottom */}
        <div className="flex-1" />

        {/* Bottom Action Buttons */}
        <div className="space-y-2 mt-4">
          {onStart && analyzer.status.status !== 'Active' && (
            <Button 
              onClick={handleStart}
              className="w-full"
              size="sm"
            >
              <Play className="h-4 w-4 mr-2" />
              Start Service
            </Button>
          )}

          {onStop && analyzer.status.status === 'Active' && (
            <Button 
              onClick={handleStop}
              className="w-full"
              size="sm"
              variant="destructive"
            >
              <Square className="h-4 w-4 mr-2" />
              Stop Service
            </Button>
          )}

          {onConfigure && analyzer.status.status !== 'Active' && (
            <Button 
              onClick={onConfigure}
              className="w-full"
              size="sm"
              variant="outline"
            >
              <Settings className="h-4 w-4 mr-2" />
              Configure
            </Button>
          )}

          <Button 
            onClick={handleViewDetails}
            className="w-full"
            size="sm"
            variant="ghost"
          >
            <ExternalLink className="h-4 w-4 mr-2" />
            View Details
          </Button>
        </div>
      </CardContent>
    </Card>
  );
} 