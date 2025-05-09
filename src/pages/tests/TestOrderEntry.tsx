import Card from '../../components/ui/Card';
import Button from '../../components/ui/Button';

const TestOrderEntry = () => {
  return (
    <div className="p-6 space-y-6">
      <h1 className="text-2xl font-bold text-gray-900">New Test Order</h1>

      <Card>
        <form className="space-y-6">
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
            <div>
              <label htmlFor="patient" className="block text-sm font-medium text-gray-700">
                Patient
              </label>
              <select
                id="patient"
                name="patient"
                className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
              >
                <option>Select a patient...</option>
              </select>
            </div>

            <div>
              <label htmlFor="machine" className="block text-sm font-medium text-gray-700">
                Machine
              </label>
              <select
                id="machine"
                name="machine"
                className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
              >
                <option>Select a machine...</option>
              </select>
            </div>

            <div>
              <label htmlFor="priority" className="block text-sm font-medium text-gray-700">
                Priority
              </label>
              <select
                id="priority"
                name="priority"
                className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
              >
                <option value="routine">Routine</option>
                <option value="urgent">Urgent</option>
                <option value="stat">STAT</option>
              </select>
            </div>

            <div>
              <label htmlFor="orderingProvider" className="block text-sm font-medium text-gray-700">
                Ordering Provider
              </label>
              <input
                type="text"
                name="orderingProvider"
                id="orderingProvider"
                className="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
              />
            </div>
          </div>

          <div>
            <label htmlFor="tests" className="block text-sm font-medium text-gray-700">
              Tests
            </label>
            <div className="mt-1 border border-gray-300 rounded-md p-4">
              <div className="space-y-2">
                {/* Test selection checkboxes would go here */}
                <p className="text-sm text-gray-500">No tests available</p>
              </div>
            </div>
          </div>

          <div>
            <label htmlFor="notes" className="block text-sm font-medium text-gray-700">
              Clinical Notes
            </label>
            <textarea
              id="notes"
              name="notes"
              rows={3}
              className="mt-1 block w-full shadow-sm sm:text-sm focus:ring-indigo-500 focus:border-indigo-500 border border-gray-300 rounded-md"
            />
          </div>

          <div className="flex justify-end space-x-3">
            <Button variant="secondary">Cancel</Button>
            <Button variant="primary">Create Order</Button>
          </div>
        </form>
      </Card>
    </div>
  );
};

export default TestOrderEntry;