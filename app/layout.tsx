import './globals.css';
import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import { Navigation } from '@/components/navigation';
import { Toaster } from '@/components/ui/toaster';
import { LabResultsListener } from './lab-results-listener';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'LIS Dashboard - Laboratory Information System',
  description: 'Real-time monitoring and management of laboratory data flows',
};

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
        <Toaster />
        <LabResultsListener />
      </body>
    </html>
  );
}