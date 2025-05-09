import { useParams } from 'react-router-dom';
import Card from '../../components/ui/Card';
import Badge from '../../components/ui/Badge';
import StatusIndicator from '../../components/ui/StatusIndicator';

function PatientDetail() {
  const { id } = useParams();

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">Patient Details</h1>
        <Badge>Active</Badge>
      </div>

      <Card>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <h3 className="text-sm font-medium text-gray-500">Patient ID</h3>
            <p className="mt-1 text-sm text-gray-900">{id}</p>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-500">Status</h3>
            <div className="mt-1">
              <StatusIndicator status="success" />
            </div>
          </div>
        </div>
      </Card>

      <Card>
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Recent Tests</h2>
        <p className="text-gray-600">No test results available.</p>
      </Card>
    </div>
  );
}

export default PatientDetail;