'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { 
  TestTube, 
  Search, 
  Calendar,
  User,
  MapPin,
  CheckCircle,
  Clock,
  AlertCircle,
  XCircle
} from 'lucide-react';
import { mockSamples } from '@/lib/mock-data';

export default function SamplesPage() {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Completed':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'InProgress':
        return <Clock className="h-4 w-4 text-blue-500 animate-pulse" />;
      case 'Error':
        return <XCircle className="h-4 w-4 text-red-500" />;
      case 'Canceled':
        return <AlertCircle className="h-4 w-4 text-yellow-500" />;
      default:
        return <Clock className="h-4 w-4 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Completed':
        return 'bg-green-100 text-green-800';
      case 'InProgress':
        return 'bg-blue-100 text-blue-800';
      case 'Error':
        return 'bg-red-100 text-red-800';
      case 'Canceled':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getSampleTypeColor = (sampleType: string) => {
    switch (sampleType) {
      case 'Blood':
        return 'bg-red-50 text-red-700 border-red-200';
      case 'Urine':
        return 'bg-yellow-50 text-yellow-700 border-yellow-200';
      case 'Serum':
        return 'bg-orange-50 text-orange-700 border-orange-200';
      case 'Plasma':
        return 'bg-purple-50 text-purple-700 border-purple-200';
      default:
        return 'bg-gray-50 text-gray-700 border-gray-200';
    }
  };

  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold">Sample Management</h1>
          <p className="text-muted-foreground">
            Track and manage laboratory samples
          </p>
        </div>
        <Button>
          <TestTube className="h-4 w-4 mr-2" />
          Register Sample
        </Button>
      </div>

      <div className="flex items-center space-x-2">
        <div className="relative flex-1 max-w-md">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search samples..."
            className="pl-10"
          />
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {mockSamples.map((sample) => (
          <Card key={sample.id} className="hover:shadow-lg transition-shadow">
            <CardHeader>
              <CardTitle className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <TestTube className="h-5 w-5 text-primary" />
                  <span>{sample.id}</span>
                </div>
                <div className="flex items-center space-x-2">
                  {getStatusIcon(sample.status)}
                  <Badge className={getStatusColor(sample.status)}>
                    {sample.status}
                  </Badge>
                </div>
              </CardTitle>
              <CardDescription>
                <Badge variant="outline" className={getSampleTypeColor(sample.sampleType)}>
                  {sample.sampleType}
                </Badge>
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {sample.containerInfo && (
                <div className="space-y-2">
                  <div className="text-sm text-muted-foreground">Container</div>
                  <div className="text-sm">
                    {sample.containerInfo.number} ({sample.containerInfo.containerType})
                  </div>
                </div>
              )}

              {sample.position && (
                <div className="space-y-2">
                  <div className="flex items-center space-x-2">
                    <MapPin className="h-4 w-4 text-muted-foreground" />
                    <span className="text-sm text-muted-foreground">Position</span>
                  </div>
                  <div className="text-sm font-mono">{sample.position}</div>
                </div>
              )}

              {sample.collection && (
                <div className="space-y-2">
                  <div className="flex items-center space-x-2">
                    <Calendar className="h-4 w-4 text-muted-foreground" />
                    <span className="text-sm text-muted-foreground">Collection</span>
                  </div>
                  <div className="text-sm">
                    {sample.collection.dateTime?.toLocaleDateString()} {sample.collection.dateTime?.toLocaleTimeString()}
                  </div>
                  {sample.collection.collectorId && (
                    <div className="flex items-center space-x-2">
                      <User className="h-4 w-4 text-muted-foreground" />
                      <span className="text-sm">{sample.collection.collectorId}</span>
                    </div>
                  )}
                </div>
              )}

              {sample.reception && (
                <div className="space-y-2">
                  <div className="text-sm text-muted-foreground">Reception</div>
                  <div className="text-sm">
                    {sample.reception.dateTime?.toLocaleDateString()} {sample.reception.dateTime?.toLocaleTimeString()}
                  </div>
                </div>
              )}

              <div className="flex space-x-2 pt-2">
                <Button variant="outline" size="sm" className="flex-1">
                  View Details
                </Button>
                <Button variant="outline" size="sm" className="flex-1">
                  Test Results
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}