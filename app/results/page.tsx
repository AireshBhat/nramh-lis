'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { 
  TestTube, 
  Search, 
  Calendar,
  Activity,
  AlertCircle,
  CheckCircle,
  Clock
} from 'lucide-react';
import { mockTestResults } from '@/lib/mock-data';

export default function TestResultsPage() {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Final':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'Preliminary':
        return <Clock className="h-4 w-4 text-yellow-500" />;
      default:
        return <AlertCircle className="h-4 w-4 text-blue-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Final':
        return 'bg-green-100 text-green-800';
      case 'Preliminary':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-blue-100 text-blue-800';
    }
  };

  const isAbnormal = (result: any) => {
    if (!result.referenceRange) return false;
    const value = parseFloat(result.value);
    if (isNaN(value)) return false;
    
    const { lowerLimit, upperLimit } = result.referenceRange;
    if (lowerLimit !== undefined && value < lowerLimit) return true;
    if (upperLimit !== undefined && value > upperLimit) return true;
    return false;
  };

  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold">Test Results</h1>
          <p className="text-muted-foreground">
            View and manage laboratory test results
          </p>
        </div>
        <Button>
          <TestTube className="h-4 w-4 mr-2" />
          New Test
        </Button>
      </div>

      <div className="flex items-center space-x-2">
        <div className="relative flex-1 max-w-md">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search test results..."
            className="pl-10"
          />
        </div>
      </div>

      <div className="grid gap-6">
        {mockTestResults.map((result) => (
          <Card key={result.id} className="hover:shadow-lg transition-shadow">
            <CardHeader>
              <CardTitle className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <TestTube className="h-5 w-5 text-primary" />
                  <span>Test ID: {result.testId}</span>
                </div>
                <div className="flex items-center space-x-2">
                  {getStatusIcon(result.status)}
                  <Badge className={getStatusColor(result.status)}>
                    {result.status}
                  </Badge>
                </div>
              </CardTitle>
              <CardDescription>
                Sample: {result.sampleId} | Sequence: {result.metadata.sequenceNumber}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                <div className="space-y-2">
                  <div className="text-sm text-muted-foreground">Test Value</div>
                  <div className="flex items-center space-x-2">
                    <span className="text-2xl font-bold">{result.value}</span>
                    {result.units && (
                      <span className="text-muted-foreground">{result.units}</span>
                    )}
                    {isAbnormal(result) && (
                      <Badge variant="destructive" className="text-xs">
                        <AlertCircle className="h-3 w-3 mr-1" />
                        Abnormal
                      </Badge>
                    )}
                  </div>
                </div>

                {result.referenceRange && (
                  <div className="space-y-2">
                    <div className="text-sm text-muted-foreground">Reference Range</div>
                    <div className="text-sm">
                      {result.referenceRange.lowerLimit} - {result.referenceRange.upperLimit} {result.units}
                    </div>
                  </div>
                )}

                <div className="space-y-2">
                  <div className="text-sm text-muted-foreground">Completed</div>
                  <div className="flex items-center space-x-2">
                    <Calendar className="h-4 w-4 text-muted-foreground" />
                    <span className="text-sm">
                      {result.completedDateTime?.toLocaleDateString()} {result.completedDateTime?.toLocaleTimeString()}
                    </span>
                  </div>
                </div>

                {result.metadata.instrument && (
                  <div className="space-y-2">
                    <div className="text-sm text-muted-foreground">Instrument</div>
                    <div className="text-sm">{result.metadata.instrument}</div>
                  </div>
                )}

                {result.analyzerId && (
                  <div className="space-y-2">
                    <div className="text-sm text-muted-foreground">Analyzer</div>
                    <div className="text-sm">{result.analyzerId}</div>
                  </div>
                )}

                {result.flags && (
                  <div className="space-y-2">
                    <div className="text-sm text-muted-foreground">Flags</div>
                    <div className="text-sm">
                      {result.flags.abnormalFlag && (
                        <Badge variant="outline" className="mr-1">
                          {result.flags.abnormalFlag}
                        </Badge>
                      )}
                      {result.flags.natureOfAbnormality && (
                        <span className="text-sm text-muted-foreground">
                          {result.flags.natureOfAbnormality}
                        </span>
                      )}
                    </div>
                  </div>
                )}
              </div>

              <div className="flex space-x-2 pt-4">
                <Button variant="outline" size="sm">
                  <Activity className="h-4 w-4 mr-2" />
                  View Details
                </Button>
                <Button variant="outline" size="sm">
                  Upload Status
                </Button>
                <Button variant="outline" size="sm">
                  Print Report
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}