import { Fragment } from 'react';
import { NavLink } from 'react-router-dom';
import { Dialog, Transition } from '@headlessui/react';
import { X, LayoutDashboard, FlaskRound as Flask, Users, ArrowRightLeft, Settings } from 'lucide-react';
import { cn } from '../../utils/cn';

interface SidebarProps {
  isOpen: boolean;
  onClose: () => void;
}

const navigation = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard },
  { 
    name: 'Lab Machines',
    href: '/machines',
    icon: Flask,
    children: [
      { name: 'Machine List', href: '/machines' },
      { name: 'Add New Machine', href: '/machines/add' }
    ]
  },
  { 
    name: 'Patient Tests',
    href: '/patients',
    icon: Users,
    children: [
      { name: 'Patients', href: '/patients' },
      { name: 'Test Results', href: '/tests/dashboard' },
      { name: 'New Test Order', href: '/tests/order' }
    ]
  },
  { 
    name: 'HIS Integration',
    href: '/integration/dashboard',
    icon: ArrowRightLeft,
    children: [
      { name: 'Dashboard', href: '/integration/dashboard' },
      { name: 'Configuration', href: '/integration/config' },
      { name: 'Transmission Queue', href: '/integration/queue' }
    ]
  },
  { 
    name: 'Administration',
    href: '/admin/users',
    icon: Settings,
    children: [
      { name: 'User Management', href: '/admin/users' },
      { name: 'Audit Log', href: '/admin/audit' }
    ]
  },
];

export default function Sidebar({ isOpen, onClose }: SidebarProps) {
  // Function to render navigation items recursively
  const renderNavItems = (items: typeof navigation) => {
    return items.map((item) => (
      <div key={item.name} className="space-y-1">
        {/* If no children, render a simple nav link */}
        {!item.children ? (
          <NavLink
            to={item.href}
            className={({ isActive }) => cn(
              'group flex items-center px-2 py-2 text-sm font-medium rounded-md',
              isActive
                ? 'bg-primary-700 text-white'
                : 'text-white hover:bg-primary-600 hover:text-white'
            )}
          >
            <item.icon className="mr-3 h-5 w-5 flex-shrink-0" aria-hidden="true" />
            {item.name}
          </NavLink>
        ) : (
          // If has children, render a collapsible section
          <div className="space-y-1">
            <button
              type="button"
              className="group flex w-full items-center px-2 py-2 text-sm font-medium rounded-md text-white hover:bg-primary-600 hover:text-white"
            >
              <item.icon className="mr-3 h-5 w-5 flex-shrink-0" aria-hidden="true" />
              <span className="flex-1 text-left">{item.name}</span>
            </button>
            <div className="ml-4 border-l border-primary-500 pl-4 space-y-1">
              {item.children.map((child) => (
                <NavLink
                  key={child.name}
                  to={child.href}
                  className={({ isActive }) => cn(
                    'group flex items-center px-2 py-2 text-sm font-medium rounded-md',
                    isActive
                      ? 'bg-primary-700 text-white'
                      : 'text-white hover:bg-primary-600 hover:text-white'
                  )}
                >
                  {child.name}
                </NavLink>
              ))}
            </div>
          </div>
        )}
      </div>
    ));
  };

  return (
    <>
      {/* Mobile sidebar */}
      <Transition.Root show={isOpen} as={Fragment}>
        <Dialog as="div" className="relative z-40 md:hidden" onClose={onClose}>
          <Transition.Child
            as={Fragment}
            enter="transition-opacity ease-linear duration-300"
            enterFrom="opacity-0"
            enterTo="opacity-100"
            leave="transition-opacity ease-linear duration-300"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="fixed inset-0 bg-gray-600 bg-opacity-75" />
          </Transition.Child>

          <div className="fixed inset-0 z-40 flex">
            <Transition.Child
              as={Fragment}
              enter="transition ease-in-out duration-300 transform"
              enterFrom="-translate-x-full"
              enterTo="translate-x-0"
              leave="transition ease-in-out duration-300 transform"
              leaveFrom="translate-x-0"
              leaveTo="-translate-x-full"
            >
              <Dialog.Panel className="relative flex w-full max-w-xs flex-1 flex-col bg-primary-800">
                <Transition.Child
                  as={Fragment}
                  enter="ease-in-out duration-300"
                  enterFrom="opacity-0"
                  enterTo="opacity-100"
                  leave="ease-in-out duration-300"
                  leaveFrom="opacity-100"
                  leaveTo="opacity-0"
                >
                  <div className="absolute top-0 right-0 -mr-12 pt-2">
                    <button
                      type="button"
                      className="ml-1 flex h-10 w-10 items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                      onClick={onClose}
                    >
                      <span className="sr-only">Close sidebar</span>
                      <X className="h-6 w-6 text-white" aria-hidden="true" />
                    </button>
                  </div>
                </Transition.Child>
                <div className="h-0 flex-1 overflow-y-auto pt-5 pb-4">
                  <div className="flex flex-shrink-0 items-center px-4">
                    <Flask className="h-8 w-8 text-white" />
                    <h1 className="ml-2 text-white font-semibold text-xl">Lab Interface</h1>
                  </div>
                  <nav className="mt-5 space-y-1 px-2">
                    {renderNavItems(navigation)}
                  </nav>
                </div>
                <div className="flex flex-shrink-0 border-t border-primary-700 p-4">
                  <div className="group block w-full flex-shrink-0">
                    <div className="flex items-center">
                      <div className="ml-3">
                        <p className="text-sm font-medium text-white">Lab Technician</p>
                        <p className="text-xs font-medium text-primary-300 group-hover:text-primary-200">
                          View profile
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </Dialog.Panel>
            </Transition.Child>
            <div className="w-14 flex-shrink-0" aria-hidden="true">
              {/* Force sidebar to shrink to fit close icon */}
            </div>
          </div>
        </Dialog>
      </Transition.Root>

      {/* Static sidebar for desktop */}
      <div className="hidden md:fixed md:inset-y-0 md:flex md:w-64 md:flex-col">
        <div className="flex min-h-0 flex-1 flex-col bg-primary-800">
          <div className="flex flex-1 flex-col overflow-y-auto pt-5 pb-4">
            <div className="flex flex-shrink-0 items-center px-4">
              <Flask className="h-8 w-8 text-white" />
              <h1 className="ml-2 text-white font-semibold text-xl">Lab Interface</h1>
            </div>
            <nav className="mt-5 flex-1 space-y-1 px-2">
              {renderNavItems(navigation)}
            </nav>
          </div>
          <div className="flex flex-shrink-0 border-t border-primary-700 p-4">
            <div className="group block w-full flex-shrink-0">
              <div className="flex items-center">
                <div>
                  <span className="inline-flex h-9 w-9 items-center justify-center rounded-full bg-primary-700 text-white">
                    <span className="text-sm font-medium leading-none">LT</span>
                  </span>
                </div>
                <div className="ml-3">
                  <p className="text-sm font-medium text-white">Lab Technician</p>
                  <p className="text-xs font-medium text-primary-300 group-hover:text-primary-200">
                    View profile
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}