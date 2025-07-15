'use client';

import { useState } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { usePathname } from 'next/navigation';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Sheet, SheetContent, SheetTrigger } from '@/components/ui/sheet';
import { 
  Activity, 
  Database, 
  Users, 
  TestTube, 
  Upload, 
  Settings,
  Menu,
  Monitor
} from 'lucide-react';

const navigation = [
  { name: 'Dashboard', href: '/', icon: Activity },
  { name: 'Analyzers', href: '/analyzers', icon: Monitor },
  { name: 'Patients', href: '/patients', icon: Users },
  { name: 'Test Results', href: '/results', icon: TestTube },
  { name: 'Samples', href: '/samples', icon: Database },
  { name: 'Upload Queue', href: '/uploads', icon: Upload },
  { name: 'Settings', href: '/settings', icon: Settings }
];

export function Navigation() {
  const pathname = usePathname();
  const [mobileOpen, setMobileOpen] = useState(false);

  const NavigationItems = () => (
    <nav className="flex flex-col space-y-2">
      {navigation.map((item) => {
        const Icon = item.icon;
        const isActive = pathname === item.href;
        
        return (
          <Link
            key={item.name}
            href={item.href}
            className={cn(
              'flex items-center space-x-3 px-3 py-2 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground',
              isActive 
                ? 'bg-primary text-primary-foreground' 
                : 'text-muted-foreground'
            )}
            onClick={() => setMobileOpen(false)}
          >
            <Icon className="h-4 w-4" />
            <span>{item.name}</span>
          </Link>
        );
      })}
    </nav>
  );

  return (
    <>
      {/* Desktop Navigation */}
      <div className="hidden md:flex md:w-64 md:flex-col md:fixed md:inset-y-0 bg-card border-r">
        <div className="flex flex-col flex-grow pt-5 pb-4 overflow-y-auto">
          <div className="flex items-center flex-shrink-0 px-4">
            <div className="flex items-center space-x-2">
              <Image 
                src="/icons/app-icon.png" 
                alt="LIS Dashboard" 
                width={24} 
                height={24} 
                className="h-6 w-6"
              />
              <span className="text-lg font-bold">LIS Dashboard</span>
            </div>
          </div>
          <div className="mt-8 flex-grow flex flex-col px-4">
            <NavigationItems />
          </div>
        </div>
      </div>

      {/* Mobile Navigation */}
      <div className="md:hidden">
        <div className="flex items-center justify-between px-4 py-3 bg-card border-b">
          <div className="flex items-center space-x-2">
            <Image 
              src="/icons/app-icon.png" 
              alt="LIS Dashboard" 
              width={24} 
              height={24} 
              className="h-6 w-6"
            />
            <span className="text-lg font-bold">LIS Dashboard</span>
          </div>
          <Sheet open={mobileOpen} onOpenChange={setMobileOpen}>
            <SheetTrigger asChild>
              <Button variant="ghost" size="icon">
                <Menu className="h-6 w-6" />
              </Button>
            </SheetTrigger>
            <SheetContent side="left" className="w-64 p-0">
              <div className="flex flex-col h-full">
                <div className="flex items-center flex-shrink-0 px-4 py-5">
                  <div className="flex items-center space-x-2">
                    <Image 
                      src="/icons/app-icon.png" 
                      alt="LIS Dashboard" 
                      width={24} 
                      height={24} 
                      className="h-6 w-6"
                    />
                    <span className="text-lg font-bold">LIS Dashboard</span>
                  </div>
                </div>
                <div className="flex-grow px-4">
                  <NavigationItems />
                </div>
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </>
  );
}