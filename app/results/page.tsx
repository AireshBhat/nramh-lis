'use client';

import { useLabResults } from '@/hooks/use-lab-results';
import { usePatients } from '@/hooks/use-patients';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { RefreshCw, User, TestTube, Calendar, Phone, MapPin } from 'lucide-react';
import { useMemo } from 'react';
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from '@/components/ui/accordion';
import { TestResult, Patient } from '@/lib/types';

export default function ResultsPage() {
  const { latestResults, allResults, clearResults, refreshResults, loading, error, clearError } = useLabResults({
    autoRefresh: true,
    refreshInterval: 10000, // Refresh every 10 seconds
    maxResultsInMemory: 100
  });

  // Fetch patients to get patient information
  const { patients, fetchPatients } = usePatients({ 
    autoLoad: true, 
    limit: 100 
  });

  // Group all test results by patient
  const patientsWithTests = useMemo(() => {
    const patientMap = new Map<string, {
      patient: {
        id: string;
        name: string;
        birthDate?: string;
        sex?: string;
        address?: string;
        telephone?: string;
        physicians?: string;
        height?: string;
        weight?: string;
      };
      tests: Array<{
        id: string;
        testId: string;
        sampleId: string;
        value: string;
        units?: string;
        referenceRange?: string;
        flags: string[];
        status: string;
        completedDateTime?: string;
        analyzerId?: string;
        timestamp: Date;
      }>;
    }>();

    // Process all results to group by patient
    allResults.forEach((testResult: TestResult) => {
      const patientId = testResult.patientId || 'unknown';
      
      // Find patient information from the patients list
      const patient = patients.find(p => p.id === patientId);
      const patientName = patient ? 
        `${patient.name.firstName || ''} ${patient.name.lastName || ''}`.trim() || 'Unknown Patient' :
        `Patient ${patientId}`;
      
      if (!patientMap.has(patientId)) {
        patientMap.set(patientId, {
          patient: {
            id: patientId,
            name: patientName,
            birthDate: patient?.birthDate?.toLocaleDateString(),
            sex: patient?.sex,
            address: patient?.address ? 
              `${patient.address.street || ''}, ${patient.address.city || ''}, ${patient.address.state || ''} ${patient.address.zip || ''}`.trim() :
              undefined,
            telephone: patient?.telephone?.join(', '),
            physicians: patient?.physicians ? 
              `${patient.physicians.ordering || ''} ${patient.physicians.attending || ''} ${patient.physicians.referring || ''}`.trim() :
              undefined,
            height: patient?.physicalAttributes?.height ? 
              `${patient.physicalAttributes.height.value} ${patient.physicalAttributes.height.unit}` :
              undefined,
            weight: patient?.physicalAttributes?.weight ? 
              `${patient.physicalAttributes.weight.value} ${patient.physicalAttributes.weight.unit}` :
              undefined,
          },
          tests: []
        });
      }

      const patientData = patientMap.get(patientId)!;
      
      // Add this test result
      patientData.tests.push({
        id: testResult.id,
        testId: testResult.testId,
        sampleId: testResult.sampleId,
        value: testResult.value,
        units: testResult.units,
        referenceRange: testResult.referenceRange ? 
          `${testResult.referenceRange.lowerLimit || ''}-${testResult.referenceRange.upperLimit || ''}` : undefined,
        flags: [
          ...(testResult.flags?.abnormalFlag ? [testResult.flags.abnormalFlag] : []),
          ...(testResult.flags?.natureOfAbnormality ? [testResult.flags.natureOfAbnormality] : [])
        ],
        status: testResult.status,
        completedDateTime: testResult.completedDateTime?.toISOString(),
        analyzerId: testResult.analyzerId,
        timestamp: testResult.completedDateTime || testResult.createdAt,
      });
    });

    // Sort patients by most recent test
    return Array.from(patientMap.values()).sort((a, b) => {
      const aLatest = Math.max(...a.tests.map(t => t.timestamp.getTime()));
      const bLatest = Math.max(...b.tests.map(t => t.timestamp.getTime()));
      return bLatest - aLatest;
    });
  }, [allResults, patients]);

  // Fetch patients when component mounts or when refreshing
  const handleRefresh = async () => {
    await Promise.all([
      refreshResults(),
      fetchPatients({ limit: 100, offset: 0 })
    ]);
  };

  return (
    <div className="container mx-auto p-6 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Patient Test Results</h1>
        <div className="flex items-center gap-2">
          {loading && (
            <div className="text-sm text-muted-foreground flex items-center gap-2">
              <RefreshCw className="h-4 w-4 animate-spin" />
              Loading...
            </div>
          )}
          <Button onClick={handleRefresh} variant="outline" disabled={loading}>
            <RefreshCw className="h-4 w-4 mr-2" />
            Refresh
          </Button>
          <Button onClick={clearResults} variant="outline">
            Clear Results
          </Button>
        </div>
      </div>

      {/* Error Display */}
      {error && (
        <div className="p-4 border border-red-200 bg-red-50 rounded-lg">
          <div className="flex justify-between items-start">
            <div className="text-red-800">
              <p className="font-medium">Error loading lab results</p>
              <p className="text-sm">{error}</p>
            </div>
            <Button onClick={clearError} variant="ghost" size="sm">
              Dismiss
            </Button>
          </div>
        </div>
      )}

      {/* Patients and Their Tests */}
      {patientsWithTests.length > 0 ? (
        <Accordion type="single" collapsible defaultValue={patientsWithTests[0]?.patient.id}>
          {patientsWithTests.map((patientData, index) => (
            <AccordionItem key={patientData.patient.id} value={patientData.patient.id}>
              <AccordionTrigger className="bg-muted/50 px-6">
                <div className="flex items-center justify-between w-full">
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-primary/10 rounded-full">
                      <User className="h-5 w-5 text-primary" />
                    </div>
                    <div>
                      <span className="block text-xl font-bold">{patientData.patient.name}</span>
                      <span className="block text-muted-foreground text-sm font-normal">
                        Patient ID: {patientData.patient.id} • {patientData.tests.length} test{patientData.tests.length !== 1 ? 's' : ''}
                      </span>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <Badge variant="outline">
                      {patientData.tests.length} tests
                    </Badge>
                    <Badge variant="secondary">
                      Latest: {Math.max(...patientData.tests.map(t => t.timestamp.getTime())).toLocaleString()}
                    </Badge>
                  </div>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <Card className="overflow-hidden border-none shadow-none">
                  <CardContent className="p-6">
                    {/* Patient Information */}
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-6 p-4 bg-muted/30 rounded-lg">
                      {patientData.patient.birthDate && (
                        <div className="flex items-center gap-2">
                          <Calendar className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Birth Date</p>
                            <p className="font-medium">{patientData.patient.birthDate}</p>
                          </div>
                        </div>
                      )}
                      {patientData.patient.sex && (
                        <div className="flex items-center gap-2">
                          <User className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Sex</p>
                            <p className="font-medium">{patientData.patient.sex}</p>
                          </div>
                        </div>
                      )}
                      {patientData.patient.telephone && (
                        <div className="flex items-center gap-2">
                          <Phone className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Telephone</p>
                            <p className="font-medium">{patientData.patient.telephone}</p>
                          </div>
                        </div>
                      )}
                      {patientData.patient.address && (
                        <div className="flex items-center gap-2 md:col-span-2 lg:col-span-1">
                          <MapPin className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Address</p>
                            <p className="font-medium">{patientData.patient.address}</p>
                          </div>
                        </div>
                      )}
                      {patientData.patient.physicians && (
                        <div className="flex items-center gap-2">
                          <User className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Physicians</p>
                            <p className="font-medium">{patientData.patient.physicians}</p>
                          </div>
                        </div>
                      )}
                      {patientData.patient.height && (
                        <div className="flex items-center gap-2">
                          <User className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Height</p>
                            <p className="font-medium">{patientData.patient.height}</p>
                          </div>
                        </div>
                      )}
                      {patientData.patient.weight && (
                        <div className="flex items-center gap-2">
                          <User className="h-4 w-4 text-muted-foreground" />
                          <div>
                            <p className="text-sm text-muted-foreground">Weight</p>
                            <p className="font-medium">{patientData.patient.weight}</p>
                          </div>
                        </div>
                      )}
                    </div>
                    <Separator className="my-6" />
                    {/* Test Results */}
                    <div className="space-y-4">
                      <div className="flex items-center gap-2 mb-4">
                        <TestTube className="h-5 w-5 text-primary" />
                        <h3 className="text-lg font-semibold">Test Results</h3>
                      </div>
                      <div className="grid gap-4">
                        {patientData.tests
                          .sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
                          .map((test, testIndex) => (
                            <div key={`${test.id}-${testIndex}`} className="p-4 border rounded-lg hover:bg-muted/30 transition-colors">
                              <div className="flex justify-between items-start mb-3">
                                <div className="space-y-1">
                                  <h4 className="font-medium text-lg">{test.testId}</h4>
                                  <p className="text-sm text-muted-foreground">
                                    Sample ID: {test.sampleId}
                                  </p>
                                  <p className="text-sm text-muted-foreground">
                                    Analyzer: {test.analyzerId || 'Unknown'}
                                  </p>
                                </div>
                                <div className="flex items-center gap-2">
                                  <Badge variant={test.status === 'Final' ? 'default' : 'secondary'}>
                                    {test.status}
                                  </Badge>
                                  <Badge variant="outline">
                                    {test.timestamp.toLocaleString()}
                                  </Badge>
                                </div>
                              </div>
                              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                <div>
                                  <p className="text-sm text-muted-foreground">Result</p>
                                  <p className="font-medium text-lg">{test.value}</p>
                                </div>
                                {test.units && (
                                  <div>
                                    <p className="text-sm text-muted-foreground">Units</p>
                                    <p className="font-medium">{test.units}</p>
                                  </div>
                                )}
                                {test.referenceRange && (
                                  <div>
                                    <p className="text-sm text-muted-foreground">Reference Range</p>
                                    <p className="font-medium">{test.referenceRange}</p>
                                  </div>
                                )}
                              </div>
                            </div>
                          ))}
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      ) : (
        <Card>
          <CardHeader>
            <CardTitle>No Test Results</CardTitle>
            <CardDescription>
              No test results have been received yet. Start the Meril analyzer service to receive results.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <p className="text-muted-foreground">
                The system will automatically display test results grouped by patient as they arrive from the analyzer.
              </p>
              <div className="text-sm text-muted-foreground">
                <p>• Make sure the Meril analyzer service is running</p>
                <p>• Check that the analyzer is connected and sending data</p>
                <p>• Test results will appear here grouped by patient when received</p>
                <p>• Data is automatically refreshed every 10 seconds from the database</p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      <div className="text-center text-sm text-muted-foreground">
        <p>Lab results are automatically stored in the database and displayed here grouped by patient.</p>
        <p>Data is refreshed from the database every 10 seconds. Check the browser console for detailed logging.</p>
      </div>
    </div>
  );
}