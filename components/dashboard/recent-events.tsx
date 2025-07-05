'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { 
  AlertCircle, 
  CheckCircle, 
  Info, 
  AlertTriangle,
  Clock
} from 'lucide-react';
import { mockSystemEvents } from '@/lib/mock-data';
import { SystemEvent } from '@/lib/types';

export function RecentEvents() {
  const [events, setEvents] = useState<SystemEvent[]>(mockSystemEvents);

  useEffect(() => {
    // Simulate real-time event updates
    const interval = setInterval(() => {
      const newEvent: SystemEvent = {
        id: `EVENT-${Date.now()}`,
        timestamp: new Date(),
        type: ['INFO', 'SUCCESS', 'WARNING', 'ERROR'][Math.floor(Math.random() * 4)] as SystemEvent['type'],
        source: ['ANALYZER', 'PROTOCOL', 'DATABASE', 'UPLOAD', 'SYSTEM'][Math.floor(Math.random() * 5)] as SystemEvent['source'],
        message: [
          'New sample received from analyzer',
          'Protocol message parsed successfully',
          'Database connection established',
          'Upload to HIS completed',
          'System health check passed'
        ][Math.floor(Math.random() * 5)],
        details: {}
      };

      setEvents(prev => [newEvent, ...prev.slice(0, 19)]);
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  const getEventIcon = (type: SystemEvent['type']) => {
    switch (type) {
      case 'SUCCESS':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'ERROR':
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      case 'WARNING':
        return <AlertTriangle className="h-4 w-4 text-yellow-500" />;
      default:
        return <Info className="h-4 w-4 text-blue-500" />;
    }
  };

  const getEventBadgeVariant = (type: SystemEvent['type']) => {
    switch (type) {
      case 'SUCCESS':
        return 'default';
      case 'ERROR':
        return 'destructive';
      case 'WARNING':
        return 'secondary';
      default:
        return 'outline';
    }
  };

  const getSourceColor = (source: SystemEvent['source']) => {
    switch (source) {
      case 'ANALYZER':
        return 'text-blue-600';
      case 'PROTOCOL':
        return 'text-purple-600';
      case 'DATABASE':
        return 'text-green-600';
      case 'UPLOAD':
        return 'text-orange-600';
      default:
        return 'text-gray-600';
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Clock className="h-5 w-5" />
          <span>Recent System Events</span>
        </CardTitle>
        <CardDescription>
          Real-time system events and notifications
        </CardDescription>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-80">
          <div className="space-y-3">
            {events.map((event) => (
              <div key={event.id} className="flex items-start space-x-3 p-3 rounded-lg border">
                <div className="flex-shrink-0 mt-0.5">
                  {getEventIcon(event.type)}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center space-x-2 mb-1">
                    <Badge variant={getEventBadgeVariant(event.type)} className="text-xs">
                      {event.type}
                    </Badge>
                    <Badge variant="outline" className={`text-xs ${getSourceColor(event.source)}`}>
                      {event.source}
                    </Badge>
                  </div>
                  <div className="text-sm font-medium">{event.message}</div>
                  <div className="text-xs text-muted-foreground">
                    {event.timestamp.toLocaleTimeString()}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}