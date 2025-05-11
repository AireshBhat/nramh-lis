import { useState } from 'react';
import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';
import BreadcrumbBar from './BreadcrumbBar';

export default function Layout() {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  
  return (
    <div className="h-full flex">
      {/* Sidebar for navigation */}
      <Sidebar 
        isOpen={sidebarOpen} 
        onClose={() => setSidebarOpen(false)} 
      />
      
      {/* Main content area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        <BreadcrumbBar />
        
        {/* Main content with scrolling */}
        <main className="flex-1 overflow-auto bg-gray-50 p-4 ml-0 md:ml-64">
          <Outlet />
        </main>
      </div>
    </div>
  );
}