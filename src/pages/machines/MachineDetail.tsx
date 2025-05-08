import React from 'react';
import { useParams } from 'react-router-dom';
import Card from '../../components/ui/Card';
import StatusIndicator from '../../components/ui/StatusIndicator';
import DataTable from '../../components/ui/DataTable';
import { Activity, Settings, AlertTriangle, CheckCircle } from 'lucide-react';

const MachineDetail = () => {
  const { id } = useParams();

  // This is a placeholder component structure
  // You'll need to implement the actual data fetching and display logic
  return (
    <div className="space-y-6 p-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">Machine Details</h1>
        <StatusIndicator status="active" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <div className="flex items-center gap-4 mb-4">
            <Settings className="w-6 h-6 text-blue-600" />
            <h2 className="text-xl font-semibold">Machine Information</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="text-sm font-medium text-gray-500">Machine ID</label>
              <p className="text-gray-900">{id}</p>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-500">Status</label>
              <p className="text-green-600 flex items-center gap-2">
                <CheckCircle className="w-4 h-4" />
                Operational
              </p>
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center gap-4 mb-4">
            <Activity className="w-6 h-6 text-blue-600" />
            <h2 className="text-xl font-semibold">Performance Metrics</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="text-sm font-medium text-gray-500">Tests Processed Today</label>
              <p className="text-gray-900">127</p>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-500">Uptime</label>
              <p className="text-gray-900">99.8%</p>
            </div>
          </div>
        </Card>
      </div>

      <Card>
        <div className="flex items-center gap-4 mb-4">
          <AlertTriangle className="w-6 h-6 text-blue-600" />
          <h2 className="text-xl font-semibold">Recent Alerts</h2>
        </div>
        <DataTable 
          columns={[
            { header: 'Date', accessor: 'date' },
            { header: 'Type', accessor: 'type' },
            { header: 'Message', accessor: 'message' },
            { header: 'Status', accessor: 'status' }
          ]}
          data={[
            { 
              date: '2025-01-20 09:15', 
              type: 'Maintenance', 
              message: 'Routine calibration required',
              status: 'Pending'
            }
          ]}
        />
      </Card>
    </div>
  );
};

export default MachineDetail;