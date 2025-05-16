import { Routes, Route } from 'react-router-dom';
import { Suspense, lazy } from 'react';
import Layout from './components/layout/Layout';
import LoadingScreen from './components/ui/LoadingScreen';
import Dashboard from './pages/Dashboard';
// import Database from '@tauri-apps/plugin-sql';

// Lazy loaded routes
// const Login = lazy(() => import('./pages/auth/Login'));
const MachineList = lazy(() => import('./pages/machines/MachineList'));
const MachineDetail = lazy(() => import('./pages/machines/MachineDetail'));
const MachineAdd = lazy(() => import('./pages/machines/MachineAdd'));
const PatientList = lazy(() => import('./pages/patients/PatientList'));
const PatientDetail = lazy(() => import('./pages/patients/PatientDetail'));
const TestResultDetail = lazy(() => import('./pages/tests/TestResultDetail'));
const TestOrderEntry = lazy(() => import('./pages/tests/TestOrderEntry'));
const ResultsDashboard = lazy(() => import('./pages/tests/ResultsDashboard'));
const IntegrationConfig = lazy(() => import('./pages/integration/IntegrationConfig'));
const TransmissionQueue = lazy(() => import('./pages/integration/TransmissionQueue'));
const TransmissionDetail = lazy(() => import('./pages/integration/TransmissionDetail'));
const TransmissionDashboard = lazy(() => import('./pages/integration/TransmissionDashboard'));
// const UserManagement = lazy(() => import('./pages/admin/UserManagement'));
// const AuditLog = lazy(() => import('./pages/admin/AuditLog'));

function App() {
  // Database.load('sqlite:mydatabase.db');
  return (
    <Routes>
      {/* <Route path="/login" element={
        <Suspense fallback={<LoadingScreen />}>
          <Login />
        </Suspense>
      } /> */}
      
      <Route path="/" element={<Layout />}>
        <Route index element={<Dashboard />} />
        
        {/* Machine Management Routes */}
        <Route path="machines" element={
          <Suspense fallback={<LoadingScreen />}>
            <MachineList />
          </Suspense>
        } />
        <Route path="machines/add" element={
          <Suspense fallback={<LoadingScreen />}>
            <MachineAdd />
          </Suspense>
        } />
        <Route path="machines/:id" element={
          <Suspense fallback={<LoadingScreen />}>
            <MachineDetail />
          </Suspense>
        } />
        
        {/* Patient/Test Results Routes */}
        <Route path="patients" element={
          <Suspense fallback={<LoadingScreen />}>
            <PatientList />
          </Suspense>
        } />
        <Route path="patients/:id" element={
          <Suspense fallback={<LoadingScreen />}>
            <PatientDetail />
          </Suspense>
        } />
        <Route path="tests/result/:id" element={
          <Suspense fallback={<LoadingScreen />}>
            <TestResultDetail />
          </Suspense>
        } />
        <Route path="tests/order" element={
          <Suspense fallback={<LoadingScreen />}>
            <TestOrderEntry />
          </Suspense>
        } />
        <Route path="tests/dashboard" element={
          <Suspense fallback={<LoadingScreen />}>
            <ResultsDashboard />
          </Suspense>
        } />
        
        {/* Integration Routes */}
        <Route path="integration/config" element={
          <Suspense fallback={<LoadingScreen />}>
            <IntegrationConfig />
          </Suspense>
        } />
        <Route path="integration/queue" element={
          <Suspense fallback={<LoadingScreen />}>
            <TransmissionQueue />
          </Suspense>
        } />
        <Route path="integration/transmission/:id" element={
          <Suspense fallback={<LoadingScreen />}>
            <TransmissionDetail />
          </Suspense>
        } />
        <Route path="integration/dashboard" element={
          <Suspense fallback={<LoadingScreen />}>
            <TransmissionDashboard />
          </Suspense>
        } />
        
        {/* Admin Routes */}
        {/* <Route path="admin/users" element={
          <Suspense fallback={<LoadingScreen />}>
            <UserManagement />
          </Suspense>
        } />
        <Route path="admin/audit" element={
          <Suspense fallback={<LoadingScreen />}>
            <AuditLog />
          </Suspense>
        } /> */}
      </Route>
    </Routes>
  );
}

export default App;