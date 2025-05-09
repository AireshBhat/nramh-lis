// import Card from '../../components/ui/Card';
// import DataTable from '../../components/ui/DataTable';
import Button from '../../components/ui/Button';
import { Link } from 'react-router-dom';
import { Plus } from 'lucide-react';

function PatientList() {
  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">Patients</h1>
        <Link to="/patients/add">
          <Button>
            <Plus className="w-4 h-4 mr-2" />
            Add Patient
          </Button>
        </Link>
      </div>

      {/* <Card>
        <DataTable 
          columns={[
            { header: 'ID', key: 'id', cell: (item) => item.id },
            { header: 'Name', key: 'name', cell: (item) => item.name },
            { header: 'DOB', key: 'dateOfBirth', cell: (item) => item.dateOfBirth },
            { header: 'Status', key: 'status', cell: (item) => item.status }
          ]}
          data={[]} // Will be populated with real data later
        />
      </Card> */}
    </div>
  );
}

export default PatientList;