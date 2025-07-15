'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { usePatients } from '@/hooks/use-patients';
import { useTestResults } from '@/hooks/use-test-results';
import { CreatePatientDTO } from '@/lib/database/types';

/**
 * Example component demonstrating database operations
 * This shows how to use the database hooks in a React component
 */
export function DatabaseExample() {
  const [activeTab, setActiveTab] = useState<'patients' | 'results'>('patients');
  
  // Database hooks
  const {
    patients,
    loading: patientsLoading,
    error: patientsError,
    createPatient,
    searchPatients,
    clearError: clearPatientsError,
  } = usePatients({ autoLoad: true, limit: 10 });

  const {
    testResults,
    statistics,
    loading: resultsLoading,
    error: resultsError,
    findAbnormalResults,
    findRecentResults,
    clearError: clearResultsError,
  } = useTestResults({ autoLoad: true, limit: 10 });

  // Example patient data
  const examplePatient: CreatePatientDTO = {
    id: '1',
    name: {
      firstName: 'John',
      lastName: 'Doe',
      title: 'Mr.',
    },
    birthDate: new Date('1990-01-01'),
    sex: 'Male',
    telephone: ['+1234567890'],
    address: {
      street: '123 Main St',
      city: 'Anytown',
      state: 'CA',
      zip: '12345',
    },
  };

  const handleCreatePatient = async () => {
    try {
      await createPatient(examplePatient);
    } catch (error) {
      console.error('Failed to create patient:', error);
    }
  };

  const handleSearchPatients = async () => {
    await searchPatients('Doe', 'John');
  };

  const handleFindAbnormalResults = async () => {
    await findAbnormalResults(5);
  };

  const handleFindRecentResults = async () => {
    await findRecentResults(24);
  };

  return (
    <div className="space-y-6">
      <div className="flex space-x-2">
        <Button
          variant={activeTab === 'patients' ? 'default' : 'outline'}
          onClick={() => setActiveTab('patients')}
        >
          Patients ({patients.length})
        </Button>
        <Button
          variant={activeTab === 'results' ? 'default' : 'outline'}
          onClick={() => setActiveTab('results')}
        >
          Test Results ({testResults.length})
        </Button>
      </div>

      {activeTab === 'patients' && (
        <Card>
          <CardHeader>
            <CardTitle>Patient Management</CardTitle>
            <CardDescription>
              Example of patient database operations
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {patientsError && (
              <Alert variant="destructive">
                <AlertDescription>
                  {patientsError}
                  <Button variant="link" onClick={clearPatientsError}>
                    Clear
                  </Button>
                </AlertDescription>
              </Alert>
            )}

            <div className="flex space-x-2">
              <Button onClick={handleCreatePatient} disabled={patientsLoading}>
                Create Example Patient
              </Button>
              <Button onClick={handleSearchPatients} disabled={patientsLoading}>
                Search "Doe, John"
              </Button>
            </div>

            {patientsLoading ? (
              <div>Loading patients...</div>
            ) : (
              <div className="space-y-2">
                <h3 className="font-semibold">Patients ({patients.length})</h3>
                {patients.map((patient) => (
                  <div key={patient.id} className="p-3 border rounded">
                    <div className="font-medium">
                      {patient.name.title} {patient.name.firstName} {patient.name.lastName}
                    </div>
                    <div className="text-sm text-gray-600">
                      ID: {patient.id} | Sex: {patient.sex}
                    </div>
                    {patient.birthDate && (
                      <div className="text-sm text-gray-600">
                        Birth: {patient.birthDate.toLocaleDateString()}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {activeTab === 'results' && (
        <Card>
          <CardHeader>
            <CardTitle>Test Results Management</CardTitle>
            <CardDescription>
              Example of test result database operations
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {resultsError && (
              <Alert variant="destructive">
                <AlertDescription>
                  {resultsError}
                  <Button variant="link" onClick={clearResultsError}>
                    Clear
                  </Button>
                </AlertDescription>
              </Alert>
            )}

            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <h3 className="font-semibold">Statistics</h3>
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div>Total: <Badge>{statistics.total}</Badge></div>
                  <div>Final: <Badge variant="secondary">{statistics.final}</Badge></div>
                  <div>Preliminary: <Badge variant="outline">{statistics.preliminary}</Badge></div>
                  <div>Abnormal: <Badge variant="destructive">{statistics.abnormal}</Badge></div>
                  <div>Today: <Badge variant="default">{statistics.today}</Badge></div>
                </div>
              </div>

              <div className="space-y-2">
                <h3 className="font-semibold">Actions</h3>
                <div className="space-y-2">
                  <Button 
                    onClick={handleFindAbnormalResults} 
                    disabled={resultsLoading}
                    size="sm"
                  >
                    Find Abnormal (5)
                  </Button>
                  <Button 
                    onClick={handleFindRecentResults} 
                    disabled={resultsLoading}
                    size="sm"
                  >
                    Recent (24h)
                  </Button>
                </div>
              </div>
            </div>

            {resultsLoading ? (
              <div>Loading test results...</div>
            ) : (
              <div className="space-y-2">
                <h3 className="font-semibold">Test Results ({testResults.length})</h3>
                {testResults.map((result) => (
                  <div key={result.id} className="p-3 border rounded">
                    <div className="flex justify-between items-start">
                      <div>
                        <div className="font-medium">
                          {result.testId} - {result.value} {result.units}
                        </div>
                        <div className="text-sm text-gray-600">
                          Sample: {result.sampleId} | Status: {result.status}
                        </div>
                      </div>
                      <div className="flex space-x-1">
                        <Badge variant={result.status === 'Final' ? 'default' : 'secondary'}>
                          {result.status}
                        </Badge>
                        {result.flags?.abnormalFlag && (
                          <Badge variant="destructive">Abnormal</Badge>
                        )}
                      </div>
                    </div>
                    {result.completedDateTime && (
                      <div className="text-sm text-gray-600 mt-1">
                        Completed: {result.completedDateTime.toLocaleString()}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
} 