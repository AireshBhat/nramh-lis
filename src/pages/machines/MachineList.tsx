import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Plus, RefreshCw, AlertTriangle, Wifi, WifiOff } from 'lucide-react';
import Card from '../../components/ui/Card';
import Button from '../../components/ui/Button';
import DataTable from '../../components/ui/DataTable';
import StatusIndicator from '../../components/ui/StatusIndicator';
import Badge from '../../components/ui/Badge';

// Sample machine data for demonstration
const SAMPLE_MACHINES = [
  { 
    id: 'AQ-101', 
    name: 'AutoQuant Chemistry A1', 
    type: 'Chemistry', 
    model: 'AutoQuant 3000',
    connection: 'COM1:9600',
    protocol: 'ASTM',
    status: 'online',
    lastCommunication: new Date(2023, 5, 15, 10, 30).toISOString(),
    pendingTests: 3,
  },
  { 
    id: 'AQ-102', 
    name: 'AutoQuant Chemistry A2', 
    type: 'Chemistry', 
    model: 'AutoQuant 3000',
    connection: 'COM2:9600',
    protocol: 'ASTM',
    status: 'offline',
    lastCommunication: new Date(2023, 5, 14, 18, 15).toISOString(),
    pendingTests: 0,
  },
  { 
    id: 'AQ-103', 
    name: 'Hematology Analyzer H1', 
    type: 'Hematology', 
    model: 'HemaCount Pro',
    connection: '192.168.1.101:3000',
    protocol: 'HL7',
    status: 'warning',
    lastCommunication: new Date(2023, 5, 15, 9, 45).toISOString(),
    pendingTests: 7,
  },
  { 
    id: 'AQ-104', 
    name: 'Urinalysis System U1', 
    type: 'Urinalysis', 
    model: 'UriScan Elite',
    connection: 'COM3:9600',
    protocol: 'ASTM',
    status: 'online',
    lastCommunication: new Date(2023, 5, 15, 10, 15).toISOString(),
    pendingTests: 2,
  },
  { 
    id: 'AQ-105', 
    name: 'Immunology Analyzer I1', 
    type: 'Immunology', 
    model: 'ImmunoQuant 2500',
    connection: '192.168.1.102:3000',
    protocol: 'HL7',
    status: 'online',
    lastCommunication: new Date(2023, 5, 15, 10, 22).toISOString(),
    pendingTests: 5,
  },
];

export default function MachineList() {
  const navigate = useNavigate();
  const [machines, setMachines] = useState(SAMPLE_MACHINES);
  const [isRefreshing, setIsRefreshing] = useState(false);
  
  // Simulate refresh action
  const handleRefresh = () => {
    setIsRefreshing(true);
    setTimeout(() => {
      setIsRefreshing(false);
    }, 1000);
  };
  
  // Table columns configuration
  const columns = [
    {
      key: 'id',
      header: 'ID',
      cell: (machine: typeof machines[0]) => (
        <span className="font-medium text-gray-900">{machine.id}</span>
      ),
    },
    {
      key: 'name',
      header: 'Machine Name',
      cell: (machine: typeof machines[0]) => (
        <div className="flex flex-col">
          <span className="font-medium text-gray-900">{machine.name}</span>
          <span className="text-xs text-gray-500">{machine.model}</span>
        </div>
      ),
    },
    {
      key: 'type',
      header: 'Type',
      cell: (machine: typeof machines[0]) => (
        <Badge variant={
          machine.type === 'Chemistry' ? 'info' :
          machine.type === 'Hematology' ? 'success' :
          machine.type === 'Urinalysis' ? 'warning' :
          'default'
        }>
          {machine.type}
        </Badge>
      ),
    },
    {
      key: 'connection',
      header: 'Connection',
      cell: (machine: typeof machines[0]) => (
        <span className="text-sm font-mono">{machine.connection}</span>
      ),
    },
    {
      key: 'status',
      header: 'Status',
      cell: (machine: typeof machines[0]) => (
        <StatusIndicator status={machine.status as any} />
      ),
    },
    {
      key: 'pendingTests',
      header: 'Pending Tests',
      cell: (machine: typeof machines[0]) => (
        <span className={`font-medium ${
          machine.pendingTests > 5 ? 'text-warning-600' : 
          machine.pendingTests > 0 ? 'text-info-600' : 
          'text-gray-500'
        }`}>
          {machine.pendingTests}
        </span>
      ),
    },
    {
      key: 'actions',
      header: 'Actions',
      cell: (machine: typeof machines[0]) => (
        <div className="flex items-center space-x-2">
          <Button 
            size="sm" 
            variant={machine.status === 'online' ? 'error' : 'success'}
            icon={machine.status === 'online' ? <WifiOff size={16} /> : <Wifi size={16} />}
          >
            {machine.status === 'online' ? 'Disconnect' : 'Connect'}
          </Button>
        </div>
      ),
    },
  ];
  
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-semibold text-gray-900">Lab Machine Management</h1>
        <div className="flex space-x-3">
          <Button 
            variant="outline" 
            icon={<RefreshCw size={16} />}
            onClick={handleRefresh}
            isLoading={isRefreshing}
          >
            Refresh Status
          </Button>
          <Button 
            icon={<Plus size={16} />}
            onClick={() => navigate('/machines/add')}
          >
            Add New Machine
          </Button>
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card className="bg-gradient-to-r from-primary-50 to-white border border-primary-100">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium text-primary-900">Online Machines</h3>
              <p className="text-3xl font-semibold text-primary-700">3</p>
            </div>
            <Wifi className="h-10 w-10 text-primary-400" />
          </div>
        </Card>
        
        <Card className="bg-gradient-to-r from-error-50 to-white border border-error-100">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium text-error-900">Offline Machines</h3>
              <p className="text-3xl font-semibold text-error-700">1</p>
            </div>
            <WifiOff className="h-10 w-10 text-error-400" />
          </div>
        </Card>
        
        <Card className="bg-gradient-to-r from-warning-50 to-white border border-warning-100">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium text-warning-900">Machines with Warnings</h3>
              <p className="text-3xl font-semibold text-warning-700">1</p>
            </div>
            <AlertTriangle className="h-10 w-10 text-warning-400" />
          </div>
        </Card>
      </div>
      
      <Card>
        <DataTable 
          columns={columns} 
          data={machines} 
          keyExtractor={(machine) => machine.id}
          onRowClick={(machine) => navigate(`/machines/${machine.id}`)}
        />
      </Card>
    </div>
  );
}