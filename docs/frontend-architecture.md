# Laboratory Information System - Frontend Architecture

## Overview

This document outlines the frontend architecture for the Laboratory Information System (LIS) built using Next.js 13+, React 18, and Tauri. The frontend follows modern web development practices with a focus on user experience, performance, and maintainability.

## Technology Stack

- **Framework**: Next.js 13+ with App Router
- **UI Library**: React 18
- **Styling**: Tailwind CSS
- **UI Components**: shadcn/ui
- **State Management**: React Hooks + Context API
- **Desktop Integration**: Tauri Commands + Events
- **Type Safety**: TypeScript

## Application Structure

### 1. Next.js App Router Structure

```
app/
├── layout.tsx          # Root layout with navigation
├── page.tsx           # Main dashboard
├── globals.css        # Global styles
├── analyzers/
│   └── page.tsx       # Analyzer management
├── patients/
│   └── page.tsx       # Patient management
├── samples/
│   └── page.tsx       # Sample tracking
├── results/
│   └── page.tsx       # Test results
├── uploads/
│   └── page.tsx       # Upload management
└── settings/
    └── page.tsx       # System settings
```

### 2. Root Layout Implementation

```typescript
// app/layout.tsx - Root layout with navigation
import { Inter } from 'next/font/google';
import Navigation from '@/components/navigation';
import './globals.css';

const inter = Inter({ subsets: ['latin'] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="min-h-screen bg-background">
          <Navigation />
          <main className="md:pl-64">
            {children}
          </main>
        </div>
      </body>
    </html>
  );
}
```

## Core Frontend Modules

### 1. Dashboard Module

#### Main Dashboard Page
```typescript
// app/page.tsx - Main dashboard
import { SystemStatus } from '@/components/dashboard/system-status';
import { DataFlowMonitor } from '@/components/dashboard/data-flow-monitor';
import { RecentEvents } from '@/components/dashboard/recent-events';

export default function Dashboard() {
  return (
    <div className="p-4 md:p-6 space-y-6">
      <SystemStatus />
      <DataFlowMonitor />
      <RecentEvents />
    </div>
  );
}
```

#### System Status Component
```typescript
// components/dashboard/system-status.tsx
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { mockMetrics } from '@/lib/mock-data';

export function SystemStatus() {
  const uptime = mockMetrics.systemUptime;
  const successRate = (mockMetrics.successfulUploads / 
    (mockMetrics.successfulUploads + mockMetrics.failedUploads)) * 100;
  
  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Samples Processed</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockMetrics.samplesProcessed}</div>
          <p className="text-xs text-muted-foreground">
            Last 24 hours
          </p>
        </CardContent>
      </Card>
      
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Upload Success Rate</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{successRate.toFixed(1)}%</div>
          <p className="text-xs text-muted-foreground">
            {mockMetrics.successfulUploads} successful uploads
          </p>
        </CardContent>
      </Card>
      
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">System Uptime</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{uptime}</div>
          <p className="text-xs text-muted-foreground">
            Continuous operation
          </p>
        </CardContent>
      </Card>
      
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Active Analyzers</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockMetrics.activeAnalyzers}</div>
          <p className="text-xs text-muted-foreground">
            Connected and operational
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
```

#### Data Flow Monitor Component
```typescript
// components/dashboard/data-flow-monitor.tsx
import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';

interface FlowStage {
  name: string;
  count: number;
  capacity: number;
  status: 'idle' | 'processing' | 'error';
}

export function DataFlowMonitor() {
  const [captureFlow, setCaptureFlow] = useState<FlowStage[]>([
    { name: 'Data Capture', count: 45, capacity: 100, status: 'processing' },
    { name: 'Validation', count: 32, capacity: 100, status: 'processing' },
    { name: 'Storage', count: 28, capacity: 100, status: 'processing' },
    { name: 'Upload Queue', count: 15, capacity: 100, status: 'processing' }
  ]);

  const [uploadFlow, setUploadFlow] = useState<FlowStage[]>([
    { name: 'Pending', count: 15, capacity: 50, status: 'processing' },
    { name: 'Uploading', count: 8, capacity: 50, status: 'processing' },
    { name: 'Completed', count: 127, capacity: 200, status: 'idle' },
    { name: 'Failed', count: 3, capacity: 50, status: 'error' }
  ]);

  useEffect(() => {
    const interval = setInterval(() => {
      // Simulate real-time updates from backend
      setCaptureFlow(prev => prev.map(stage => ({
        ...stage,
        count: Math.max(0, stage.count + (Math.random() > 0.5 ? 1 : -1))
      })));
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="grid gap-6 md:grid-cols-2">
      <Card>
        <CardHeader>
          <CardTitle>Laboratory Data Capture</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {captureFlow.map((stage, index) => (
            <div key={index} className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>{stage.name}</span>
                <span>{stage.count}/{stage.capacity}</span>
              </div>
              <Progress value={(stage.count / stage.capacity) * 100} />
            </div>
          ))}
        </CardContent>
      </Card>
      
      <Card>
        <CardHeader>
          <CardTitle>HIS Upload Process</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {uploadFlow.map((stage, index) => (
            <div key={index} className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>{stage.name}</span>
                <span>{stage.count}/{stage.capacity}</span>
              </div>
              <Progress value={(stage.count / stage.capacity) * 100} />
            </div>
          ))}
        </CardContent>
      </Card>
    </div>
  );
}
```

### 2. Analyzer Management Module

#### Analyzer Page Implementation
```typescript
// app/analyzers/page.tsx
'use client';

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { IPAddressBanner } from '@/components/analyzers/ip-address-banner';
import { AnalyzerGrid } from '@/components/analyzers/analyzer-grid';
import { mockAnalyzers } from '@/lib/mock-data';
import type { Analyzer } from '@/lib/types';

export default function AnalyzersPage() {
  const [localIp, setLocalIp] = useState<string>('Loading...');
  const [analyzers, setAnalyzers] = useState<Analyzer[]>([]);

  useEffect(() => {
    const getLocalIp = async () => {
      try {
        const ip = await invoke('get_local_ip');
        setLocalIp(ip as string);
      } catch (error) {
        console.warn('Could not fetch local IP:', error);
        setLocalIp('192.168.1.100'); // Fallback
      }
    };
    getLocalIp();
  }, []);

  return (
    <div className="p-4 md:p-6 space-y-6">
      <IPAddressBanner localIp={localIp} />
      <AnalyzerGrid analyzers={mockAnalyzers} />
    </div>
  );
}
```

### 3. Patient Management Module

#### Patient Page Implementation
```typescript
// app/patients/page.tsx
import { PatientSearchInterface } from '@/components/patients/patient-search-interface';
import { PatientGrid } from '@/components/patients/patient-grid';
import { mockPatients } from '@/lib/mock-data';
import type { Patient, PatientName } from '@/lib/types';

export default function PatientsPage() {
  const formatName = (name: PatientName) => {
    const parts = [];
    if (name.title) parts.push(name.title);
    if (name.firstName) parts.push(name.firstName);
    if (name.middleName) parts.push(name.middleName);
    if (name.lastName) parts.push(name.lastName);
    return parts.join(' ');
  };

  return (
    <div className="p-4 md:p-6 space-y-6">
      <PatientSearchInterface />
      <PatientGrid patients={mockPatients} />
    </div>
  );
}
```

## Component Architecture

### 1. Shared UI Components

#### Navigation Component
```typescript
// components/navigation.tsx
'use client';

import { useState } from 'react';
import { usePathname } from 'next/navigation';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Sheet, SheetContent, SheetTrigger } from '@/components/ui/sheet';
import { Menu } from 'lucide-react';

interface NavigationItem {
  name: string;
  href: string;
  icon: React.ComponentType<{ className?: string }>;
}

const navigation: NavigationItem[] = [
  { name: 'Dashboard', href: '/', icon: HomeIcon },
  { name: 'Analyzers', href: '/analyzers', icon: SettingsIcon },
  { name: 'Patients', href: '/patients', icon: UsersIcon },
  { name: 'Samples', href: '/samples', icon: TestTubeIcon },
  { name: 'Results', href: '/results', icon: FileTextIcon },
  { name: 'Uploads', href: '/uploads', icon: UploadIcon },
  { name: 'Settings', href: '/settings', icon: CogIcon },
];

export function Navigation() {
  const pathname = usePathname();
  const [mobileOpen, setMobileOpen] = useState(false);

  return (
    <>
      {/* Desktop Navigation */}
      <div className="hidden md:flex md:w-64 md:flex-col md:fixed md:inset-y-0">
        <div className="flex flex-col flex-grow pt-5 bg-white overflow-y-auto border-r border-gray-200">
          <div className="flex items-center flex-shrink-0 px-4">
            <h1 className="text-xl font-semibold">LIS System</h1>
          </div>
          <div className="mt-5 flex-grow flex flex-col">
            <nav className="flex-1 px-2 pb-4 space-y-1">
              {navigation.map((item) => {
                const isActive = pathname === item.href;
                return (
                  <a
                    key={item.name}
                    href={item.href}
                    className={cn(
                      isActive
                        ? 'bg-gray-100 text-gray-900'
                        : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900',
                      'group flex items-center px-2 py-2 text-sm font-medium rounded-md'
                    )}
                  >
                    <item.icon
                      className={cn(
                        isActive ? 'text-gray-500' : 'text-gray-400 group-hover:text-gray-500',
                        'mr-3 flex-shrink-0 h-6 w-6'
                      )}
                    />
                    {item.name}
                  </a>
                );
              })}
            </nav>
          </div>
        </div>
      </div>

      {/* Mobile Navigation */}
      <Sheet open={mobileOpen} onOpenChange={setMobileOpen}>
        <SheetTrigger asChild>
          <Button
            variant="ghost"
            className="md:hidden"
            size="sm"
          >
            <Menu className="h-6 w-6" />
          </Button>
        </SheetTrigger>
        <SheetContent side="left" className="w-64">
          <ScrollArea className="my-4 h-[calc(100vh-8rem)] pb-10">
            <div className="flex items-center px-2">
              <h1 className="text-xl font-semibold">LIS System</h1>
            </div>
            <div className="mt-5 space-y-1">
              {navigation.map((item) => {
                const isActive = pathname === item.href;
                return (
                  <a
                    key={item.name}
                    href={item.href}
                    className={cn(
                      isActive
                        ? 'bg-gray-100 text-gray-900'
                        : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900',
                      'group flex items-center px-2 py-2 text-sm font-medium rounded-md'
                    )}
                    onClick={() => setMobileOpen(false)}
                  >
                    <item.icon
                      className={cn(
                        isActive ? 'text-gray-500' : 'text-gray-400 group-hover:text-gray-500',
                        'mr-3 flex-shrink-0 h-6 w-6'
                      )}
                    />
                    {item.name}
                  </a>
                );
              })}
            </div>
          </ScrollArea>
        </SheetContent>
      </Sheet>
    </>
  );
}
```

### 2. Form Components

#### Search Interface
```typescript
// components/patients/patient-search-interface.tsx
'use client';

import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Search, Plus } from 'lucide-react';

export function PatientSearchInterface() {
  const [searchTerm, setSearchTerm] = useState('');

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // Implement search logic
    console.log('Searching for:', searchTerm);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Patient Search</CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSearch} className="flex gap-2">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
            <Input
              type="text"
              placeholder="Search by name, ID, or date of birth..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="pl-10"
            />
          </div>
          <Button type="submit">Search</Button>
          <Button type="button" variant="outline">
            <Plus className="h-4 w-4 mr-2" />
            Add Patient
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}
```

## State Management

### 1. React Hooks and Context

#### Application Context
```typescript
// lib/context/app-context.tsx
'use client';

import { createContext, useContext, useReducer, ReactNode } from 'react';

interface AppState {
  theme: 'light' | 'dark';
  notifications: Notification[];
  user: User | null;
}

interface AppAction {
  type: string;
  payload?: any;
}

const initialState: AppState = {
  theme: 'light',
  notifications: [],
  user: null,
};

function appReducer(state: AppState, action: AppAction): AppState {
  switch (action.type) {
    case 'SET_THEME':
      return { ...state, theme: action.payload };
    case 'ADD_NOTIFICATION':
      return { 
        ...state, 
        notifications: [...state.notifications, action.payload] 
      };
    case 'REMOVE_NOTIFICATION':
      return {
        ...state,
        notifications: state.notifications.filter(n => n.id !== action.payload)
      };
    case 'SET_USER':
      return { ...state, user: action.payload };
    default:
      return state;
  }
}

const AppContext = createContext<{
  state: AppState;
  dispatch: React.Dispatch<AppAction>;
} | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(appReducer, initialState);

  return (
    <AppContext.Provider value={{ state, dispatch }}>
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error('useApp must be used within an AppProvider');
  }
  return context;
}
```

### 2. Custom Hooks

#### Data Fetching Hooks
```typescript
// hooks/use-analyzers.ts
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import type { Analyzer } from '@/lib/types';

export function useAnalyzers() {
  const [analyzers, setAnalyzers] = useState<Analyzer[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchAnalyzers = async () => {
      try {
        setLoading(true);
        const result = await invoke('get_analyzers');
        setAnalyzers(result as Analyzer[]);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch analyzers');
      } finally {
        setLoading(false);
      }
    };

    fetchAnalyzers();
  }, []);

  return { analyzers, loading, error };
}

// hooks/use-patients.ts
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import type { Patient } from '@/lib/types';

export function usePatients(searchTerm?: string) {
  const [patients, setPatients] = useState<Patient[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchPatients = async () => {
      try {
        setLoading(true);
        const result = await invoke('get_patients', { searchTerm });
        setPatients(result as Patient[]);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch patients');
      } finally {
        setLoading(false);
      }
    };

    fetchPatients();
  }, [searchTerm]);

  return { patients, loading, error };
}
```

## Tauri Integration

### 1. Command Invocation

#### Backend Communication
```typescript
// lib/tauri-commands.ts
import { invoke } from '@tauri-apps/api/tauri';

export const tauriCommands = {
  // Analyzer commands
  getAnalyzers: () => invoke('get_analyzers'),
  createAnalyzer: (analyzer: any) => invoke('create_analyzer', { analyzer }),
  updateAnalyzer: (id: string, analyzer: any) => invoke('update_analyzer', { id, analyzer }),
  deleteAnalyzer: (id: string) => invoke('delete_analyzer', { id }),
  
  // Patient commands
  getPatients: (searchTerm?: string) => invoke('get_patients', { searchTerm }),
  createPatient: (patient: any) => invoke('create_patient', { patient }),
  updatePatient: (id: string, patient: any) => invoke('update_patient', { id, patient }),
  deletePatient: (id: string) => invoke('delete_patient', { id }),
  
  // System commands
  getLocalIp: () => invoke('get_local_ip'),
  getSystemHealth: () => invoke('get_system_health'),
  getSystemMetrics: () => invoke('get_system_metrics'),
  
  // Upload commands
  getUploadStatus: () => invoke('get_upload_status'),
  retryUpload: (id: string) => invoke('retry_upload', { id }),
  cancelUpload: (id: string) => invoke('cancel_upload', { id }),
};
```

### 2. Event Listening

#### Real-time Updates
```typescript
// lib/tauri-events.ts
import { listen } from '@tauri-apps/api/event';

export const setupEventListeners = () => {
  // Listen for analyzer status changes
  listen('analyzer-status-changed', (event) => {
    console.log('Analyzer status changed:', event.payload);
    // Update UI accordingly
  });

  // Listen for new test results
  listen('new-test-result', (event) => {
    console.log('New test result:', event.payload);
    // Update dashboard metrics
  });

  // Listen for upload status changes
  listen('upload-status-changed', (event) => {
    console.log('Upload status changed:', event.payload);
    // Update upload queue
  });

  // Listen for system health updates
  listen('system-health-update', (event) => {
    console.log('System health update:', event.payload);
    // Update system status
  });
};
```

## Performance Optimization

### 1. Data Fetching and Caching

#### SWR Integration
```typescript
// hooks/use-swr-analyzers.ts
import useSWR from 'swr';
import { tauriCommands } from '@/lib/tauri-commands';

export function useAnalyzers() {
  return useSWR('/api/analyzers', tauriCommands.getAnalyzers, {
    refreshInterval: 30000, // 30 seconds
    revalidateOnFocus: false,
    revalidateOnReconnect: true,
  });
}

export function usePatients(searchTerm?: string) {
  return useSWR(
    searchTerm ? `/api/patients?search=${searchTerm}` : '/api/patients',
    () => tauriCommands.getPatients(searchTerm),
    {
      refreshInterval: 60000, // 1 minute
      revalidateOnFocus: false,
    }
  );
}
```

### 2. Virtual Scrolling

#### Large Dataset Handling
```typescript
// components/ui/virtualized-list.tsx
import { FixedSizeList as List } from 'react-window';
import { useVirtualizer } from '@tanstack/react-virtual';

interface VirtualizedListProps<T> {
  items: T[];
  height: number;
  itemHeight: number;
  renderItem: (item: T, index: number) => React.ReactNode;
}

export function VirtualizedList<T>({ 
  items, 
  height, 
  itemHeight, 
  renderItem 
}: VirtualizedListProps<T>) {
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: items.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => itemHeight,
  });

  return (
    <div ref={parentRef} style={{ height, overflow: 'auto' }}>
      <div
        style={{
          height: `${virtualizer.getTotalSize()}px`,
          width: '100%',
          position: 'relative',
        }}
      >
        {virtualizer.getVirtualItems().map((virtualItem) => (
          <div
            key={virtualItem.key}
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              width: '100%',
              height: `${itemHeight}px`,
              transform: `translateY(${virtualItem.start}px)`,
            }}
          >
            {renderItem(items[virtualItem.index], virtualItem.index)}
          </div>
        ))}
      </div>
    </div>
  );
}
```

## Error Handling

### 1. Error Boundaries

#### Global Error Boundary
```typescript
// components/error-boundary.tsx
'use client';

import { Component, ReactNode } from 'react';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { AlertTriangle } from 'lucide-react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: any) {
    console.error('Error caught by boundary:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="flex items-center justify-center min-h-screen p-4">
          <Alert className="max-w-md">
            <AlertTriangle className="h-4 w-4" />
            <AlertDescription>
              <div className="space-y-4">
                <p>Something went wrong. Please try refreshing the page.</p>
                <Button 
                  onClick={() => window.location.reload()}
                  variant="outline"
                >
                  Refresh Page
                </Button>
              </div>
            </AlertDescription>
          </Alert>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### 2. Toast Notifications

#### Notification System
```typescript
// components/ui/toast-notifications.tsx
import { useToast } from '@/hooks/use-toast';

export function useNotification() {
  const { toast } = useToast();

  return {
    success: (message: string) => {
      toast({
        title: 'Success',
        description: message,
        variant: 'default',
      });
    },
    error: (message: string) => {
      toast({
        title: 'Error',
        description: message,
        variant: 'destructive',
      });
    },
    warning: (message: string) => {
      toast({
        title: 'Warning',
        description: message,
        variant: 'default',
      });
    },
    info: (message: string) => {
      toast({
        title: 'Information',
        description: message,
        variant: 'default',
      });
    },
  };
}
```

## Testing Strategy

### 1. Component Testing

#### Unit Tests
```typescript
// __tests__/components/system-status.test.tsx
import { render, screen } from '@testing-library/react';
import { SystemStatus } from '@/components/dashboard/system-status';

describe('SystemStatus', () => {
  test('renders system metrics correctly', () => {
    render(<SystemStatus />);
    expect(screen.getByText(/samples processed/i)).toBeInTheDocument();
    expect(screen.getByText(/upload success rate/i)).toBeInTheDocument();
    expect(screen.getByText(/system uptime/i)).toBeInTheDocument();
    expect(screen.getByText(/active analyzers/i)).toBeInTheDocument();
  });

  test('displays correct metric values', () => {
    render(<SystemStatus />);
    expect(screen.getByText('1,234')).toBeInTheDocument(); // samples processed
    expect(screen.getByText('98.5%')).toBeInTheDocument(); // success rate
  });
});
```

### 2. Integration Testing

#### Page Testing
```typescript
// __tests__/pages/analyzers.test.tsx
import { render, screen, waitFor } from '@testing-library/react';
import AnalyzersPage from '@/app/analyzers/page';

// Mock Tauri commands
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn(),
}));

describe('AnalyzersPage', () => {
  test('displays IP address banner', async () => {
    render(<AnalyzersPage />);
    
    await waitFor(() => {
      expect(screen.getByText(/local ip address/i)).toBeInTheDocument();
    });
  });

  test('displays analyzer grid', () => {
    render(<AnalyzersPage />);
    expect(screen.getByText(/analyzers/i)).toBeInTheDocument();
  });
});
```

## Development Workflow

### 1. Local Development Setup

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Run tests
npm run test

# Build for production
npm run build

# Start Tauri development
npm run tauri dev
```

### 2. Code Quality Tools

```json
// package.json scripts
{
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "type-check": "tsc --noEmit",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage"
  }
}
```

## Future Enhancements

### 1. Advanced UI Features

- **Real-time Charts**: Integration with Chart.js or D3.js for advanced data visualization
- **Drag and Drop**: File upload interfaces with drag and drop functionality
- **Keyboard Shortcuts**: Power user features with keyboard navigation
- **Progressive Web App**: PWA capabilities for offline functionality

### 2. Performance Improvements

- **Code Splitting**: Dynamic imports for better bundle optimization
- **Service Workers**: Caching strategies for improved performance
- **Web Workers**: Background processing for heavy computations
- **Image Optimization**: Next.js Image component integration

### 3. Accessibility Enhancements

- **Screen Reader Support**: ARIA labels and semantic HTML
- **Keyboard Navigation**: Full keyboard accessibility
- **High Contrast Mode**: Theme support for accessibility
- **Focus Management**: Proper focus handling for modals and forms

### 4. Internationalization

- **Multi-language Support**: i18n integration with next-intl
- **RTL Support**: Right-to-left language support
- **Localized Formatting**: Date, time, and number formatting
- **Cultural Adaptations**: Region-specific UI patterns

This frontend architecture provides a solid foundation for building a modern, performant, and maintainable laboratory information system interface. 