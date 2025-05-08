import { ReactNode } from 'react';
import { cn } from '../../utils/cn';

interface CardProps {
  children: ReactNode;
  className?: string;
  title?: string;
  subtitle?: string;
  actions?: ReactNode;
  noPadding?: boolean;
}

export default function Card({ children, className, title, subtitle, actions, noPadding = false }: CardProps) {
  return (
    <div className={cn(
      "bg-white rounded-lg shadow-card overflow-hidden",
      className
    )}>
      {(title || actions) && (
        <div className="px-6 py-4 flex justify-between items-center border-b border-gray-200">
          <div>
            {title && <h3 className="text-lg font-medium text-gray-900">{title}</h3>}
            {subtitle && <p className="mt-1 text-sm text-gray-500">{subtitle}</p>}
          </div>
          {actions && <div className="flex space-x-2">{actions}</div>}
        </div>
      )}
      <div className={noPadding ? '' : 'p-6'}>
        {children}
      </div>
    </div>
  );
}