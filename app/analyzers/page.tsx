'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { 
  Monitor, 
  Wifi, 
  Settings, 
  Activity,
  AlertCircle,
  CheckCircle,
  Wrench,
  Server,
  Globe,
  Copy,
  Check
} from 'lucide-react';
import { mockAnalyzers } from '@/lib/mock-data';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function AnalyzersPage() {
  const [localIp, setLocalIp] = useState<string>('Loading...');
  const [isLoading, setIsLoading] = useState(true);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    const getLocalIp = async () => {
      try {
        const ip = await invoke('get_local_ip');
        setLocalIp(ip as string);
      } catch (error) {
        console.warn('Could not fetch local IP:', error);
        setLocalIp('192.168.1.100'); // Fallback
      } finally {
        setIsLoading(false);
      }
    };

    getLocalIp();
  }, []);

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

  const copyToClipboard = async () => {
    if (localIp && localIp !== 'Loading...') {
      try {
        await navigator.clipboard.writeText(localIp);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000); // Reset after 2 seconds
      } catch (error) {
        console.error('Failed to copy IP address:', error);
      }
    }
  };

  return (
    <div className="p-4 md:p-6 space-y-6">
      {/* Local System IP Banner */}
      <Alert className="border-blue-200 bg-blue-50">
        <Server className="h-4 w-4 text-blue-600" />
        <AlertDescription className="text-blue-800">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              <Globe className="h-4 w-4" />
              <div>
                <span className="font-medium">Local System IP Address</span>
                <p className="text-xs text-blue-600 mt-1">
                  This IP address is used by analyzers to connect to the LIS system
                </p>
              </div>
            </div>
            <div className="flex items-center space-x-2">
              <Wifi className="h-4 w-4 text-blue-600" />
              <div className="flex items-center space-x-1">
                <span className="font-mono text-sm bg-blue-100 px-2 py-1 rounded">
                  {isLoading ? 'Loading...' : localIp}
                </span>
                {!isLoading && localIp !== 'Loading...' && (
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={copyToClipboard}
                    className="h-6 w-6 p-0 hover:bg-blue-200"
                    title="Copy IP address"
                  >
                    {copied ? (
                      <Check className="h-3 w-3 text-green-600" />
                    ) : (
                      <Copy className="h-3 w-3 text-blue-600" />
                    )}
                  </Button>
                )}
              </div>
            </div>
          </div>
        </AlertDescription>
      </Alert>

      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold">Laboratory Analyzers</h1>
          <p className="text-muted-foreground">
            Manage and monitor laboratory analyzer connections
          </p>
        </div>
        <Button>
          <Settings className="h-4 w-4 mr-2" />
          Add Analyzer
        </Button>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {mockAnalyzers.map((analyzer) => (
          <Card key={analyzer.id} className="relative">
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

              <div className="flex space-x-2 pt-2">
                <Button variant="outline" size="sm" className="flex-1">
                  <Settings className="h-4 w-4 mr-2" />
                  Configure
                </Button>
                <Button variant="outline" size="sm" className="flex-1">
                  <Activity className="h-4 w-4 mr-2" />
                  Monitor
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}