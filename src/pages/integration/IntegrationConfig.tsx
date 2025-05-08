import React from 'react';
import Card from '../../components/ui/Card';
import { Settings } from 'lucide-react';

const IntegrationConfig: React.FC = () => {
  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center gap-3 mb-6">
        <Settings className="w-6 h-6 text-blue-600" />
        <h1 className="text-2xl font-semibold text-gray-900">Integration Configuration</h1>
      </div>
      
      <Card>
        <div className="p-6">
          <h2 className="text-lg font-medium text-gray-900 mb-4">Integration Settings</h2>
          <p className="text-gray-600">Configure your integration settings here.</p>
        </div>
      </Card>
    </div>
  );
};

export default IntegrationConfig;