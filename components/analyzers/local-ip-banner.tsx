import { Alert, AlertDescription } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Server, Globe, Wifi, Copy, Check } from 'lucide-react';
import { useState } from 'react';

interface LocalIpBannerProps {
  localIp: string;
  isLoading: boolean;
}

export function LocalIpBanner({ localIp, isLoading }: LocalIpBannerProps) {
  const [copied, setCopied] = useState(false);

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
  );
} 