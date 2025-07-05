'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { 
  Upload, 
  RefreshCw, 
  CheckCircle, 
  AlertCircle, 
  Clock,
  XCircle,
  Activity
} from 'lucide-react';
import { mockUploadStatus } from '@/lib/mock-data';

export default function UploadsPage() {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Uploaded':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'Uploading':
        return <Activity className="h-4 w-4 text-blue-500 animate-pulse" />;
      case 'Failed':
        return <XCircle className="h-4 w-4 text-red-500" />;
      default:
        return <Clock className="h-4 w-4 text-yellow-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Uploaded':
        return 'bg-green-100 text-green-800';
      case 'Uploading':
        return 'bg-blue-100 text-blue-800';
      case 'Failed':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-yellow-100 text-yellow-800';
    }
  };

  const getProgressValue = (status: string) => {
    switch (status) {
      case 'Uploaded':
        return 100;
      case 'Uploading':
        return 60;
      case 'Failed':
        return 0;
      default:
        return 0;
    }
  };

  const totalUploads = mockUploadStatus.length;
  const successfulUploads = mockUploadStatus.filter(u => u.status === 'Uploaded').length;
  const failedUploads = mockUploadStatus.filter(u => u.status === 'Failed').length;
  const pendingUploads = mockUploadStatus.filter(u => u.status === 'Pending').length;

  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold">HIS Upload Queue</h1>
          <p className="text-muted-foreground">
            Monitor and manage uploads to Hospital Information System
          </p>
        </div>
        <Button>
          <RefreshCw className="h-4 w-4 mr-2" />
          Refresh Queue
        </Button>
      </div>

      {/* Upload Statistics */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Total Uploads</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalUploads}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Successful</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">{successfulUploads}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Failed</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">{failedUploads}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Pending</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600">{pendingUploads}</div>
          </CardContent>
        </Card>
      </div>

      {/* Upload Queue */}
      <div className="space-y-4">
        <h2 className="text-xl font-semibold">Upload Queue</h2>
        
        <div className="grid gap-4">
          {mockUploadStatus.map((upload) => (
            <Card key={upload.id} className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <CardTitle className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <Upload className="h-5 w-5 text-primary" />
                    <span>Upload ID: {upload.id}</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    {getStatusIcon(upload.status)}
                    <Badge className={getStatusColor(upload.status)}>
                      {upload.status}
                    </Badge>
                  </div>
                </CardTitle>
                <CardDescription>
                  Result: {upload.resultId} | System: {upload.externalSystemId}
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span>Upload Progress</span>
                    <span>{getProgressValue(upload.status)}%</span>
                  </div>
                  <div className="mt-2 w-full bg-secondary rounded-full h-4">
                    <div 
                      className="h-full bg-primary rounded-full transition-all" 
                      style={{ width: `${getProgressValue(upload.status)}%` }}
                    />
                  </div>
                </div>

                <div className="grid gap-4 md:grid-cols-3">
                  <div className="space-y-2">
                    <div className="text-sm text-muted-foreground">Created</div>
                    <div className="text-sm">
                      {upload.createdAt.toLocaleDateString()} {upload.createdAt.toLocaleTimeString()}
                    </div>
                  </div>

                  {upload.uploadDate && (
                    <div className="space-y-2">
                      <div className="text-sm text-muted-foreground">Upload Date</div>
                      <div className="text-sm">
                        {upload.uploadDate.toLocaleDateString()} {upload.uploadDate.toLocaleTimeString()}
                      </div>
                    </div>
                  )}

                  <div className="space-y-2">
                    <div className="text-sm text-muted-foreground">Retry Count</div>
                    <div className="text-sm">{upload.retryCount}</div>
                  </div>

                  {upload.responseCode && (
                    <div className="space-y-2">
                      <div className="text-sm text-muted-foreground">Response Code</div>
                      <Badge variant="outline">{upload.responseCode}</Badge>
                    </div>
                  )}

                  {upload.responseMessage && (
                    <div className="space-y-2 md:col-span-2">
                      <div className="text-sm text-muted-foreground">Response Message</div>
                      <div className="text-sm">{upload.responseMessage}</div>
                    </div>
                  )}
                </div>

                <div className="flex space-x-2 pt-2">
                  <Button variant="outline" size="sm">
                    View Details
                  </Button>
                  {upload.status === 'Failed' && (
                    <Button variant="outline" size="sm">
                      <RefreshCw className="h-4 w-4 mr-2" />
                      Retry Upload
                    </Button>
                  )}
                  <Button variant="outline" size="sm">
                    <Activity className="h-4 w-4 mr-2" />
                    View Logs
                  </Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
}