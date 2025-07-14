'use client';

import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { 
  Users, 
  Search, 
  Plus,
  Calendar,
  Phone,
  MapPin,
  UserCheck,
  Loader2
} from 'lucide-react';
import { usePatients } from '@/hooks/use-patients';

export default function PatientsPage() {
  const [searchQuery, setSearchQuery] = useState('');
  const { 
    patients, 
    loading, 
    error, 
    searchPatients, 
    clearError 
  } = usePatients({ autoLoad: true, limit: 50 });

  const formatName = (name: any) => {
    const parts = [];
    if (name.title) parts.push(name.title);
    if (name.firstName) parts.push(name.firstName);
    if (name.middleName) parts.push(name.middleName);
    if (name.lastName) parts.push(name.lastName);
    return parts.join(' ');
  };

  const formatAddress = (address: any) => {
    if (!address) return 'No address provided';
    const parts = [];
    if (address.street) parts.push(address.street);
    if (address.city) parts.push(address.city);
    if (address.state) parts.push(address.state);
    if (address.zip) parts.push(address.zip);
    return parts.join(', ');
  };

  const calculateAge = (birthDate: Date) => {
    const today = new Date();
    const age = today.getFullYear() - birthDate.getFullYear();
    const monthDiff = today.getMonth() - birthDate.getMonth();
    
    if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
      return age - 1;
    }
    return age;
  };

  const handleSearch = async () => {
    if (searchQuery.trim()) {
      // Simple search - you might want to enhance this based on your needs
      const query = searchQuery.trim();
      const parts = query.split(' ');
      const lastName = parts[0];
      const firstName = parts.length > 1 ? parts[1] : undefined;
      await searchPatients(lastName, firstName);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold">Patient Management</h1>
          <p className="text-muted-foreground">
            View and manage patient information
          </p>
        </div>
        <Button>
          <Plus className="h-4 w-4 mr-2" />
          Add Patient
        </Button>
      </div>

      {error && (
        <div className="bg-destructive/15 border border-destructive/25 text-destructive px-4 py-3 rounded-md flex items-center justify-between">
          <span>{error}</span>
          <Button variant="ghost" size="sm" onClick={clearError}>
            Dismiss
          </Button>
        </div>
      )}

      <div className="flex items-center space-x-2">
        <div className="relative flex-1 max-w-md">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search patients by name..."
            className="pl-10"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyPress={handleKeyPress}
          />
        </div>
        <Button onClick={handleSearch} disabled={loading}>
          {loading ? (
            <Loader2 className="h-4 w-4 animate-spin" />
          ) : (
            <Search className="h-4 w-4" />
          )}
        </Button>
      </div>

      {loading && patients.length === 0 ? (
        <div className="flex items-center justify-center py-12">
          <div className="flex items-center space-x-2">
            <Loader2 className="h-6 w-6 animate-spin" />
            <span>Loading patients...</span>
          </div>
        </div>
      ) : patients.length === 0 ? (
        <div className="text-center py-12">
          <Users className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
          <h3 className="text-lg font-medium mb-2">No patients found</h3>
          <p className="text-muted-foreground">
            {searchQuery ? 'Try adjusting your search terms' : 'Get started by adding your first patient'}
          </p>
        </div>
      ) : (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {patients.map((patient) => (
            <Card key={patient.id} className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <CardTitle className="flex items-center space-x-2">
                  <UserCheck className="h-5 w-5 text-primary" />
                  <span>{formatName(patient.name)}</span>
                </CardTitle>
                <CardDescription>
                  Patient ID: {patient.id}
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <div className="flex items-center space-x-2">
                      <Calendar className="h-4 w-4 text-muted-foreground" />
                      <span className="text-sm text-muted-foreground">Age</span>
                    </div>
                    <div className="text-sm font-medium">
                      {patient.birthDate ? calculateAge(patient.birthDate) : 'N/A'} years
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <div className="flex items-center space-x-2">
                      <Users className="h-4 w-4 text-muted-foreground" />
                      <span className="text-sm text-muted-foreground">Gender</span>
                    </div>
                    <Badge variant="outline">{patient.sex}</Badge>
                  </div>
                </div>

                {patient.telephone && patient.telephone.length > 0 && (
                  <div className="space-y-2">
                    <div className="flex items-center space-x-2">
                      <Phone className="h-4 w-4 text-muted-foreground" />
                      <span className="text-sm text-muted-foreground">Phone</span>
                    </div>
                    <div className="text-sm">{patient.telephone[0]}</div>
                  </div>
                )}

                <div className="space-y-2">
                  <div className="flex items-center space-x-2">
                    <MapPin className="h-4 w-4 text-muted-foreground" />
                    <span className="text-sm text-muted-foreground">Address</span>
                  </div>
                  <div className="text-sm">{formatAddress(patient.address)}</div>
                </div>

                {patient.physicians && (
                  <div className="space-y-2">
                    <div className="flex items-center space-x-2">
                      <UserCheck className="h-4 w-4 text-muted-foreground" />
                      <span className="text-sm text-muted-foreground">Physicians</span>
                    </div>
                    <div className="space-y-1">
                      {patient.physicians.ordering && (
                        <div className="text-sm">
                          <span className="text-muted-foreground">Ordering:</span> {patient.physicians.ordering}
                        </div>
                      )}
                      {patient.physicians.attending && (
                        <div className="text-sm">
                          <span className="text-muted-foreground">Attending:</span> {patient.physicians.attending}
                        </div>
                      )}
                    </div>
                  </div>
                )}

                <div className="flex space-x-2 pt-2">
                  <Button variant="outline" size="sm" className="flex-1">
                    View Details
                  </Button>
                  <Button variant="outline" size="sm" className="flex-1">
                    Test History
                  </Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}