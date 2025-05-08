import React from 'react';
import { Link } from 'react-router-dom';
import Card from '../../components/ui/Card';
import DataTable from '../../components/ui/DataTable';
import StatusIndicator from '../../components/ui/StatusIndicator';

const ResultsDashboard = () => {
  return (
    <div className="p-6 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">Results Dashboard</h1>
        <Link
          to="/tests/order"
          className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          New Test Order
        </Link>
      </div>

      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <Card>
          <div className="px-4 py-5 sm:p-6">
            <dt className="text-sm font-medium text-gray-500 truncate">Total Tests Today</dt>
            <dd className="mt-1 text-3xl font-semibold text-gray-900">0</dd>
          </div>
        </Card>

        <Card>
          <div className="px-4 py-5 sm:p-6">
            <dt className="text-sm font-medium text-gray-500 truncate">Pending Results</dt>
            <dd className="mt-1 text-3xl font-semibold text-gray-900">0</dd>
          </div>
        </Card>

        <Card>
          <div className="px-4 py-5 sm:p-6">
            <dt className="text-sm font-medium text-gray-500 truncate">Critical Results</dt>
            <dd className="mt-1 text-3xl font-semibold text-gray-900">0</dd>
          </div>
        </Card>

        <Card>
          <div className="px-4 py-5 sm:p-6">
            <dt className="text-sm font-medium text-gray-500 truncate">Completed Tests</dt>
            <dd className="mt-1 text-3xl font-semibold text-gray-900">0</dd>
          </div>
        </Card>
      </div>

      <Card>
        <div className="px-4 py-5 sm:p-6">
          <h2 className="text-lg font-medium text-gray-900">Recent Results</h2>
          <div className="mt-4">
            <DataTable
              columns={[
                { header: 'Test ID', accessor: 'id' },
                { header: 'Patient', accessor: 'patient' },
                { header: 'Test Type', accessor: 'type' },
                { header: 'Status', accessor: 'status', 
                  cell: (value) => <StatusIndicator status={value} />
                },
                { header: 'Date', accessor: 'date' },
              ]}
              data={[]}
              emptyMessage="No recent test results"
            />
          </div>
        </div>
      </Card>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <Card>
          <div className="px-4 py-5 sm:p-6">
            <h2 className="text-lg font-medium text-gray-900">Tests by Machine</h2>
            <div className="mt-4">
              <DataTable
                columns={[
                  { header: 'Machine', accessor: 'machine' },
                  { header: 'Tests Today', accessor: 'testsToday' },
                  { header: 'Status', accessor: 'status',
                    cell: (value) => <StatusIndicator status={value} />
                  },
                ]}
                data={[]}
                emptyMessage="No machine data available"
              />
            </div>
          </div>
        </Card>

        <Card>
          <div className="px-4 py-5 sm:p-6">
            <h2 className="text-lg font-medium text-gray-900">Pending Orders</h2>
            <div className="mt-4">
              <DataTable
                columns={[
                  { header: 'Order ID', accessor: 'id' },
                  { header: 'Patient', accessor: 'patient' },
                  { header: 'Priority', accessor: 'priority' },
                  { header: 'Ordered By', accessor: 'orderedBy' },
                ]}
                data={[]}
                emptyMessage="No pending orders"
              />
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
};

export default ResultsDashboard;