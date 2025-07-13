'use client';

import { useTestResults } from '@/hooks/use-test-results';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';

export default function ResultsPage() {
  const { latestResults, allResults, clearResults } = useTestResults();

  return (
    <div className="container mx-auto p-6 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Test Results</h1>
        <Button onClick={clearResults} variant="outline">
          Clear Results
        </Button>
      </div>

      {latestResults && (
        <Card>
          <CardHeader>
            <CardTitle>Latest Test Results</CardTitle>
            <CardDescription>
              Most recent test results from the Meril analyzer
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Patient Information */}
            {latestResults.patientData && (
              <div className="space-y-4">
                <h3 className="text-lg font-semibold">Patient Information</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 p-4 bg-muted/50 rounded-lg">
                  <div>
                    <p className="text-sm text-muted-foreground">Name</p>
                    <p className="font-medium">{latestResults.patientData.name}</p>
                  </div>
                  <div>
                    <p className="text-sm text-muted-foreground">Patient ID</p>
                    <p className="font-medium">{latestResults.patientId || 'Not provided'}</p>
                  </div>
                  {latestResults.patientData.birth_date && (
                    <div>
                      <p className="text-sm text-muted-foreground">Birth Date</p>
                      <p className="font-medium">{latestResults.patientData.birth_date}</p>
                    </div>
                  )}
                  {latestResults.patientData.sex && (
                    <div>
                      <p className="text-sm text-muted-foreground">Sex</p>
                      <p className="font-medium">{latestResults.patientData.sex}</p>
                    </div>
                  )}
                  {latestResults.patientData.address && (
                    <div className="md:col-span-2">
                      <p className="text-sm text-muted-foreground">Address</p>
                      <p className="font-medium">{latestResults.patientData.address}</p>
                    </div>
                  )}
                  {latestResults.patientData.telephone && (
                    <div>
                      <p className="text-sm text-muted-foreground">Telephone</p>
                      <p className="font-medium">{latestResults.patientData.telephone}</p>
                    </div>
                  )}
                  {latestResults.patientData.physicians && (
                    <div>
                      <p className="text-sm text-muted-foreground">Physicians</p>
                      <p className="font-medium">{latestResults.patientData.physicians}</p>
                    </div>
                  )}
                  {latestResults.patientData.height && (
                    <div>
                      <p className="text-sm text-muted-foreground">Height</p>
                      <p className="font-medium">{latestResults.patientData.height}</p>
                    </div>
                  )}
                  {latestResults.patientData.weight && (
                    <div>
                      <p className="text-sm text-muted-foreground">Weight</p>
                      <p className="font-medium">{latestResults.patientData.weight}</p>
                    </div>
                  )}
                </div>
              </div>
            )}

            <Separator />

            {/* Test Results */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold">Test Results</h3>
                <div className="flex items-center gap-2">
                  <Badge variant="outline">
                    {latestResults.timestamp.toLocaleString()}
                  </Badge>
                  <Badge variant="default">
                    {latestResults.testResults.length} tests
                  </Badge>
                </div>
              </div>
              
              <div className="grid gap-3">
                {latestResults.testResults.map((test, index) => (
                  <div key={index} className="p-4 border rounded-lg">
                    <div className="flex justify-between items-start mb-2">
                      <div>
                        <h4 className="font-medium">{test.testId}</h4>
                        <p className="text-sm text-muted-foreground">
                          Sample ID: {test.sampleId}
                        </p>
                      </div>
                      <Badge variant={test.status === 'Final' ? 'default' : 'secondary'}>
                        {test.status}
                      </Badge>
                    </div>
                    
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                      <div>
                        <p className="text-muted-foreground">Value</p>
                        <p className="font-medium">{test.value} {test.units}</p>
                      </div>
                                             {test.referenceRange && (
                         <div>
                           <p className="text-muted-foreground">Reference Range</p>
                           <p className="font-medium">
                             {typeof test.referenceRange === 'string' 
                               ? test.referenceRange 
                               : `${test.referenceRange.lowerLimit || ''}-${test.referenceRange.upperLimit || ''}`
                             }
                           </p>
                         </div>
                       )}
                      {test.flags && (
                        <div>
                          <p className="text-muted-foreground">Flags</p>
                          <p className="font-medium">{test.flags.abnormalFlag || 'None'}</p>
                        </div>
                      )}
                      {test.completedDateTime && (
                        <div>
                          <p className="text-muted-foreground">Completed</p>
                          <p className="font-medium">{test.completedDateTime.toLocaleString()}</p>
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {!latestResults && (
        <Card>
          <CardHeader>
            <CardTitle>No Test Results</CardTitle>
            <CardDescription>
              No test results have been received yet. Start the Meril analyzer service to receive results.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              The system will automatically display test results and patient information as they arrive from the analyzer.
            </p>
          </CardContent>
        </Card>
      )}

      {/* Results History */}
      {allResults.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Results History</CardTitle>
            <CardDescription>
              Complete history of test results ({allResults.length} batches)
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {allResults.map((batch, batchIndex) => (
                <div key={batchIndex} className="p-3 border rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <Badge variant="outline">Batch {batchIndex + 1}</Badge>
                    <span className="text-sm text-muted-foreground">
                      {batch.timestamp.toLocaleString()}
                    </span>
                    <Badge variant="secondary">
                      {batch.testResults.length} tests
                    </Badge>
                    {batch.patientData && (
                      <Badge variant="default">
                        {batch.patientData.name}
                      </Badge>
                    )}
                  </div>
                  <div className="grid gap-1 text-sm">
                    {batch.testResults.map((test, testIndex) => (
                      <div key={testIndex} className="text-muted-foreground">
                        {test.testId}: {test.value} {test.units} ({test.status})
                      </div>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      <div className="text-center text-sm text-muted-foreground">
        <p>Check the browser console for detailed logging of test results and patient information.</p>
        <p>The hook automatically logs all received data with comprehensive formatting.</p>
      </div>
    </div>
  );
}