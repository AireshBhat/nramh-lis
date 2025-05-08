import { useState } from 'react';
import { Menu, Bell, Search, MenuIcon } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

interface HeaderProps {
  onMenuButtonClick: () => void;
}

export default function Header({ onMenuButtonClick }: HeaderProps) {
  const [showNotifications, setShowNotifications] = useState(false);
  
  // Sample notifications for demonstration
  const notifications = [
    { id: 1, message: 'Critical result for Patient #12345', type: 'critical', time: '5 min ago' },
    { id: 2, message: 'Connection error with AutoQuant A1', type: 'error', time: '10 min ago' },
    { id: 3, message: 'Daily calibration due for Machine XYZ', type: 'warning', time: '30 min ago' },
    { id: 4, message: '15 new results awaiting review', type: 'info', time: '1 hour ago' },
  ];
  
  // Get appropriate color based on notification type
  const getNotificationColor = (type: string) => {
    switch (type) {
      case 'critical': return 'bg-error-100 text-error-800 border-l-4 border-error-500';
      case 'error': return 'bg-error-100 text-error-800 border-l-4 border-error-500';
      case 'warning': return 'bg-warning-100 text-warning-800 border-l-4 border-warning-500';
      case 'info': return 'bg-info-100 text-info-800 border-l-4 border-info-500';
      default: return 'bg-gray-100 text-gray-800';
    }
  };
  
  return (
    <header className="bg-white shadow-sm z-10">
      <div className="mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex h-16 justify-between">
          <div className="flex">
            <button
              type="button"
              className="border-r border-gray-200 px-4 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-primary-500 md:hidden"
              onClick={onMenuButtonClick}
            >
              <span className="sr-only">Open sidebar</span>
              <MenuIcon className="h-6 w-6" aria-hidden="true" />
            </button>
            <div className="flex flex-1 items-center md:ml-6">
              <div className="w-full max-w-lg lg:max-w-xs">
                <label htmlFor="search" className="sr-only">Search</label>
                <div className="relative text-gray-400 focus-within:text-gray-600">
                  <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
                    <Search className="h-5 w-5" aria-hidden="true" />
                  </div>
                  <input
                    id="search"
                    className="block w-full rounded-md border-0 bg-white py-1.5 pl-10 pr-3 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-primary-500 sm:text-sm sm:leading-6"
                    placeholder="Search patients, tests, machines..."
                    type="search"
                  />
                </div>
              </div>
            </div>
          </div>
          <div className="flex items-center">
            <div className="relative ml-4">
              <button
                type="button"
                className="relative rounded-full bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                onClick={() => setShowNotifications(!showNotifications)}
              >
                <span className="sr-only">View notifications</span>
                <Bell className="h-6 w-6" aria-hidden="true" />
                {/* Notification badge */}
                <span className="absolute top-0 right-0 block h-2 w-2 rounded-full bg-error-500 ring-2 ring-white"></span>
              </button>
              
              {/* Notification dropdown */}
              <AnimatePresence>
                {showNotifications && (
                  <motion.div 
                    initial={{ opacity: 0, y: -10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -10 }}
                    transition={{ duration: 0.2 }}
                    className="absolute right-0 z-10 mt-2 w-80 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                  >
                    <div className="px-4 py-2 border-b border-gray-100">
                      <h3 className="text-sm font-medium text-gray-900">Notifications</h3>
                    </div>
                    <div className="max-h-96 overflow-y-auto py-1">
                      {notifications.map((notification) => (
                        <a
                          key={notification.id}
                          href="#"
                          className={`block px-4 py-2 text-sm ${getNotificationColor(notification.type)}`}
                        >
                          <div className="flex justify-between">
                            <p className="font-medium">{notification.message}</p>
                            <p className="text-xs text-gray-500">{notification.time}</p>
                          </div>
                        </a>
                      ))}
                    </div>
                    <div className="border-t border-gray-100 px-4 py-2">
                      <button
                        className="text-sm text-primary-600 hover:text-primary-800 font-medium"
                      >
                        View all notifications
                      </button>
                    </div>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
            
            {/* Profile dropdown */}
            <Menu as="div" className="relative ml-4">
              <div>
                <Menu.Button className="relative flex rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2">
                  <span className="sr-only">Open user menu</span>
                  <span className="inline-flex h-8 w-8 items-center justify-center rounded-full bg-primary-600 text-white">
                    <span className="text-sm font-medium leading-none">LT</span>
                  </span>
                </Menu.Button>
              </div>
            </Menu>
          </div>
        </div>
      </div>
    </header>
  );
}