import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { 
  Monitor, 
  Wifi, 
  AlertCircle, 
  CheckCircle, 
  Wrench, 
  Globe,
  MapPin,
  Calendar,
  Hash,
  Cpu,
  Network
} from 'lucide-react';
import { Analyzer } from '@/lib/types';
import { useLocalIp } from '@/hooks/use-local-ip';

interface AnalyzerDetailsCardProps {
  analyzer: Analyzer;
}

export function AnalyzerDetailsCard({ analyzer }: AnalyzerDetailsCardProps) {
  const { localIp } = useLocalIp();

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

  const formatDate = (dateString: string) => {
    try {
      return new Date(dateString).toLocaleString();
    } catch {
      return 'Unknown';
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Monitor className="h-5 w-5" />
          <span>Analyzer Details</span>
        </CardTitle>
        <CardDescription>
          Complete configuration and status information
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Basic Information */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-muted-foreground">Basic Information</h4>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Name</span>
            <span className="text-sm font-medium">{analyzer.name}</span>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Manufacturer</span>
            <span className="text-sm font-medium">{analyzer.manufacturer || 'N/A'}</span>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Model</span>
            <span className="text-sm font-medium">{analyzer.model}</span>
          </div>

          {analyzer.serialNumber && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Serial Number</span>
              <span className="text-sm font-mono">{analyzer.serialNumber}</span>
            </div>
          )}
        </div>

        <Separator />

        {/* Status Information */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-muted-foreground">Status</h4>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Current Status</span>
            <div className="flex items-center space-x-2">
              {getStatusIcon(analyzer.status.status)}
              <Badge className={getStatusColor(analyzer.status.status)}>
                {analyzer.status.status}
              </Badge>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Auto Start</span>
            <Badge variant={analyzer.activateOnStart ? 'default' : 'outline'}>
              {analyzer.activateOnStart ? 'Enabled' : 'Disabled'}
            </Badge>
          </div>
        </div>

        <Separator />

        {/* Connection Information */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-muted-foreground flex items-center space-x-2">
            <Network className="h-4 w-4" />
            <span>Connection</span>
          </h4>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Type</span>
            <div className="flex items-center space-x-2">
              <Wifi className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm font-medium">{analyzer.connectionType.type}</span>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Protocol</span>
            <Badge variant="outline">{analyzer.protocol.protocol}</Badge>
          </div>

          {analyzer.port && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Local Listener</span>
              <div className="flex items-center space-x-2">
                <MapPin className="h-3 w-3 text-muted-foreground" />
                <span className="text-sm font-mono">{localIp}:{analyzer.port}</span>
              </div>
            </div>
          )}

          {analyzer.external_ip && analyzer.external_port && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">External LIS</span>
              <div className="flex items-center space-x-2">
                <Globe className="h-3 w-3 text-muted-foreground" />
                <span className="text-sm font-mono">{analyzer.external_ip}:{analyzer.external_port}</span>
              </div>
            </div>
          )}

          {analyzer.comPort && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">COM Port</span>
              <span className="text-sm font-mono">{analyzer.comPort}</span>
            </div>
          )}

          {analyzer.baudRate && (
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Baud Rate</span>
              <span className="text-sm font-mono">{analyzer.baudRate}</span>
            </div>
          )}
        </div>

        <Separator />

        {/* System Information */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-muted-foreground flex items-center space-x-2">
            <Cpu className="h-4 w-4" />
            <span>System</span>
          </h4>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">ID</span>
            <div className="flex items-center space-x-2">
              <Hash className="h-3 w-3 text-muted-foreground" />
              <span className="text-sm font-mono">{analyzer.id}</span>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Created</span>
            <div className="flex items-center space-x-2">
              <Calendar className="h-3 w-3 text-muted-foreground" />
              <span className="text-xs">{formatDate(analyzer.createdAt)}</span>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Last Updated</span>
            <div className="flex items-center space-x-2">
              <Calendar className="h-3 w-3 text-muted-foreground" />
              <span className="text-xs">{formatDate(analyzer.updatedAt)}</span>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}