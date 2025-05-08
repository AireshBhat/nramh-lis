import { cn } from '../../utils/cn';

interface StatusIndicatorProps {
  status: 'online' | 'offline' | 'warning' | 'processing' | 'error' | 'success';
  label?: string;
  size?: 'sm' | 'md' | 'lg';
}

export default function StatusIndicator({ status, label, size = 'md' }: StatusIndicatorProps) {
  const getStatusColor = () => {
    switch (status) {
      case 'online':
      case 'success':
        return 'bg-success-500';
      case 'offline':
      case 'error':
        return 'bg-error-500';
      case 'warning':
        return 'bg-warning-500';
      case 'processing':
        return 'bg-info-500 animate-pulse';
      default:
        return 'bg-gray-500';
    }
  };
  
  const getStatusLabel = () => {
    if (label) return label;
    
    switch (status) {
      case 'online': return 'Online';
      case 'offline': return 'Offline';
      case 'warning': return 'Warning';
      case 'processing': return 'Processing';
      case 'error': return 'Error';
      case 'success': return 'Success';
      default: return 'Unknown';
    }
  };
  
  const dotSize = {
    sm: 'h-2 w-2',
    md: 'h-3 w-3',
    lg: 'h-4 w-4',
  };
  
  const textSize = {
    sm: 'text-xs',
    md: 'text-sm',
    lg: 'text-base',
  };

  return (
    <div className="flex items-center">
      <div
        className={cn(
          'rounded-full mr-1.5',
          dotSize[size],
          getStatusColor()
        )}
      />
      <span className={cn('font-medium', textSize[size])}>
        {getStatusLabel()}
      </span>
    </div>
  );
}