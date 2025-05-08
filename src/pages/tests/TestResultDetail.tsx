import React from 'react';
import { useParams } from 'react-router-dom';
import Card from '../../components/ui/Card';
import StatusIndicator from '../../components/ui/StatusIndicator';

const TestResultDetail = () => {
  const { id } = useParams();

  return (
    <div className="p-6 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">Test Result Details</h1>
        <StatusIndicator status="completed" />
      </div>

      <Card>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <h3 className="text-sm font-medium text-gray-500">Test ID</h3>
            <p className="mt-1 text-sm text-gray-900">{id}</p>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-500">Test Date</h3>
            <p className="mt-1 text-sm text-gray-900">Loading...</p>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-500">Patient</h3>
            <p className="mt-1 text-sm text-gray-900">Loading...</p>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-500">Machine</h3>
            <p className="mt-1 text-sm text-gray-900">Loading...</p>
          </div>
        </div>
      </Card>

      <Card>
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Results</h2>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead>
              <tr>
                <th className="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Test</th>
                <th className="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Result</th>
                <th className="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Reference Range</th>
                <th className="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              <tr>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">Loading...</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">Loading...</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">Loading...</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">Loading...</td>
              </tr>
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
};

export default TestResultDetail;