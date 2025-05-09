import { useParams } from 'react-router-dom';
import Card from '../../components/ui/Card';
import { FileText } from 'lucide-react';

const TransmissionDetail: React.FC = () => {
  const { id } = useParams();

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center gap-3 mb-6">
        <FileText className="w-6 h-6 text-blue-600" />
        <h1 className="text-2xl font-semibold text-gray-900">Transmission Details</h1>
      </div>
      
      <Card>
        <div className="p-6">
          <h2 className="text-lg font-medium text-gray-900 mb-4">Transmission {id}</h2>
          <p className="text-gray-600">View transmission details here.</p>
        </div>
      </Card>
    </div>
  );
};

export default TransmissionDetail;