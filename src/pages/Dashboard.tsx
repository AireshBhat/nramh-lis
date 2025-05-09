import { Line, Bar, Doughnut } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  ArcElement,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js';
import { Activity, AlertCircle, CheckCircle2, ClipboardCheck, Clock, ExternalLink, FlaskRound as Flask } from 'lucide-react';

import Card from '../components/ui/Card';
import Button from '../components/ui/Button';
import StatusIndicator, { Status } from '../components/ui/StatusIndicator';

// Register ChartJS components
ChartJS.register(
  ArcElement,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  Filler
);

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'bottom' as const,
    },
  },
  scales: {
    y: {
      beginAtZero: true,
    },
  },
  elements: {
    line: {
      tension: 0.4,
    },
  },
};

export default function Dashboard() {
  // For demonstration purposes - this would be fetched from an API
  const testVolumeData = {
    labels: ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'],
    datasets: [
      {
        label: 'Test Volume',
        data: [65, 78, 90, 81, 86, 55, 40],
        fill: true,
        backgroundColor: 'rgba(37, 99, 235, 0.1)',
        borderColor: 'rgba(37, 99, 235, 1)',
      },
    ],
  };
  
  const testTypeData = {
    labels: ['Chemistry', 'Hematology', 'Urinalysis', 'Microbiology', 'Serology'],
    datasets: [
      {
        label: 'Test Types',
        data: [30, 25, 15, 10, 20],
        backgroundColor: [
          'rgba(37, 99, 235, 0.7)',  // Primary
          'rgba(13, 148, 136, 0.7)', // Secondary 
          'rgba(16, 185, 129, 0.7)', // Success
          'rgba(245, 158, 11, 0.7)', // Warning
          'rgba(239, 68, 68, 0.7)',  // Error
        ],
        borderWidth: 1,
      },
    ],
  };
  
  const turnaroundTimeData = {
    labels: ['Chemistry', 'Hematology', 'Urinalysis', 'Microbiology', 'Serology'],
    datasets: [
      {
        label: 'Average Turnaround Time (minutes)',
        data: [45, 30, 15, 180, 60],
        backgroundColor: 'rgba(13, 148, 136, 0.7)',
      },
    ],
  };
  
  // Sample system status for demonstration
  const systemStatus = [
    { name: 'Chemistry Analyzer A1', status: 'online' },
    { name: 'Hematology Analyzer H3', status: 'warning' },
    { name: 'Urinalysis System U2', status: 'offline' },
    { name: 'HIS Integration', status: 'online' },
    { name: 'Database Backup', status: 'success' },
  ];
  
  // Sample pending actions
  const pendingActions = [
    { type: 'critical', message: '3 critical results pending approval' },
    { type: 'warning', message: 'Daily calibration due for Analyzer A1' },
    { type: 'info', message: '15 new results ready for review' },
    { type: 'info', message: 'Software update available' },
  ];
  
  // Sample KPI metrics
  const kpiMetrics = [
    { label: 'Tests Today', value: 245, icon: <ClipboardCheck className="h-6 w-6 text-primary-600" />, change: '+15%' },
    { label: 'Active Machines', value: 8, icon: <Flask className="h-6 w-6 text-secondary-600" />, change: '+1' },
    { label: 'Avg. Turnaround', value: '42min', icon: <Clock className="h-6 w-6 text-warning-600" />, change: '-5min' },
    { label: 'Success Rate', value: '99.2%', icon: <CheckCircle2 className="h-6 w-6 text-success-600" />, change: '+0.5%' },
  ];
  
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {kpiMetrics.map((metric, index) => (
          <Card key={index} className="flex items-center p-6">
            <div className="mr-4 bg-gray-100 p-3 rounded-full">
              {metric.icon}
            </div>
            <div>
              <p className="text-sm font-medium text-gray-500">{metric.label}</p>
              <p className="text-2xl font-semibold text-gray-900">{metric.value}</p>
              <p className="text-xs text-success-600">{metric.change}</p>
            </div>
          </Card>
        ))}
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Test Volume Chart */}
        <Card title="Test Volume (Last 7 Days)" className="h-100">
          <div className="h-full flex items-center justify-center">
            <Line data={testVolumeData} options={chartOptions} height={200} />
          </div>
        </Card>
        
        {/* Test Distribution by Type */}
        <Card title="Test Distribution by Type" className="h-100">
          <div className="h-full flex items-center justify-center">
            <div className="h-64 w-64">
              <Doughnut data={testTypeData} />
            </div>
          </div>
        </Card>
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* System Status */}
        <Card 
          title="System Status" 
          className="lg:col-span-1"
          actions={
            <Button variant="outline" size="sm" icon={<ExternalLink size={16} />}>
              Details
            </Button>
          }
        >
          <div className="space-y-3">
            {systemStatus.map((system, index) => (
              <div key={index} className="flex justify-between items-center py-2 border-b border-gray-100 last:border-0">
                <span className="text-sm font-medium text-gray-800">{system.name}</span>
                <StatusIndicator status={system.status as Status} />
              </div>
            ))}
          </div>
        </Card>
        
        {/* Pending Actions */}
        <Card 
          title="Pending Actions" 
          className="lg:col-span-1"
          actions={
            <Button variant="outline" size="sm">
              View All
            </Button>
          }
        >
          <div className="space-y-2">
            {pendingActions.map((action, index) => {
              const getActionIcon = () => {
                switch (action.type) {
                  case 'critical': return <AlertCircle className="h-5 w-5 text-error-600" />;
                  case 'warning': return <AlertCircle className="h-5 w-5 text-warning-600" />;
                  case 'info': return <Activity className="h-5 w-5 text-info-600" />;
                  default: return <Activity className="h-5 w-5 text-gray-600" />;
                }
              };
              
              return (
                <div key={index} className="flex p-2 rounded-md hover:bg-gray-50">
                  <div className="mr-3">{getActionIcon()}</div>
                  <div className="text-sm text-gray-700">{action.message}</div>
                </div>
              );
            })}
          </div>
        </Card>
        
        {/* Turnaround Time */}
        <Card title="Average Turnaround Time by Test Type" className="lg:col-span-1">
          <Bar 
            data={turnaroundTimeData} 
            options={{
              ...chartOptions,
              indexAxis: 'y' as const,
            }} 
            height={200} 
          />
        </Card>
      </div>
    </div>
  );
}