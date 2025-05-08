import { useLocation, Link } from 'react-router-dom';
import { ChevronRight, Home } from 'lucide-react';

export default function BreadcrumbBar() {
  const location = useLocation();
  const pathnames = location.pathname.split('/').filter((x) => x);
  
  // Create breadcrumb mapping for better display names
  const getBreadcrumbName = (path: string) => {
    const mapping: Record<string, string> = {
      'machines': 'Lab Machines',
      'patients': 'Patients',
      'tests': 'Test Results',
      'integration': 'Integration',
      'admin': 'Administration',
      'users': 'User Management',
      'audit': 'Audit Log',
      'config': 'Configuration',
      'queue': 'Transmission Queue',
      'dashboard': 'Dashboard',
      'add': 'Add New',
      'order': 'New Test Order',
      'result': 'Test Result',
      'transmission': 'Transmission'
    };
    
    return mapping[path] || path;
  };
  
  // Handle numeric IDs in breadcrumbs (usually item details)
  const formatPathname = (pathname: string, index: number) => {
    // If path segment is numeric and we have a previous segment, format it nicely
    if (!isNaN(Number(pathname)) && index > 0) {
      const previousSegment = pathnames[index - 1];
      const entityName = previousSegment.endsWith('s') 
        ? previousSegment.slice(0, -1) 
        : previousSegment;
      
      return `${getBreadcrumbName(entityName)} #${pathname}`;
    }
    
    return getBreadcrumbName(pathname);
  };

  return (
    <nav className="bg-white border-b border-gray-200 px-4 py-2.5 text-sm text-gray-600" aria-label="Breadcrumb">
      <ol className="flex items-center space-x-1">
        <li className="flex items-center">
          <Link to="/" className="hover:text-primary-600 flex items-center">
            <Home className="h-4 w-4 flex-shrink-0" />
            <span className="sr-only">Home</span>
          </Link>
        </li>
        
        {pathnames.map((pathname, index) => {
          const routeTo = `/${pathnames.slice(0, index + 1).join('/')}`;
          
          return (
            <li key={pathname} className="flex items-center">
              <ChevronRight className="h-4 w-4 flex-shrink-0 text-gray-400" />
              {index === pathnames.length - 1 ? (
                <span className="ml-1 font-medium text-gray-800" aria-current="page">
                  {formatPathname(pathname, index)}
                </span>
              ) : (
                <Link 
                  to={routeTo} 
                  className="ml-1 hover:text-primary-600"
                >
                  {formatPathname(pathname, index)}
                </Link>
              )}
            </li>
          );
        })}
      </ol>
    </nav>
  );
}