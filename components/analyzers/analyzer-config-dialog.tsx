'use client';

import { useState, useEffect } from 'react';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Loader2, AlertCircle, Network, Server } from 'lucide-react';
import { Analyzer } from '@/lib/types';

interface AnalyzerConfigDialogProps {
  analyzer: Analyzer | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSave: (updatedAnalyzer: Partial<Analyzer>) => Promise<void>;
  loading?: boolean;
}

export function AnalyzerConfigDialog({ 
  analyzer, 
  open, 
  onOpenChange, 
  onSave, 
  loading = false 
}: AnalyzerConfigDialogProps) {
  const [formData, setFormData] = useState<Partial<Analyzer>>({});
  const [errors, setErrors] = useState<Record<string, string>>({});

  // Initialize form data when analyzer changes
  useEffect(() => {
    if (analyzer) {
      setFormData({
        name: analyzer.name,
        model: analyzer.model,
        serialNumber: analyzer.serialNumber,
        manufacturer: analyzer.manufacturer,
        ipAddress: analyzer.ipAddress,
        port: analyzer.port,
        external_ip: analyzer.external_ip,
        external_port: analyzer.external_port,
        activateOnStart: analyzer.activateOnStart,
      });
    }
  }, [analyzer]);

  // Validation functions
  const validateIpAddress = (ip: string): boolean => {
    const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
    return ipRegex.test(ip);
  };

  const validatePort = (port: number): boolean => {
    return port >= 1 && port <= 65535;
  };

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    // Validate external IP if provided
    if (formData.external_ip && !validateIpAddress(formData.external_ip)) {
      newErrors.external_ip = 'Please enter a valid IP address (e.g., 192.168.1.100)';
    }

    // Validate external port if provided
    if (formData.external_port && !validatePort(formData.external_port)) {
      newErrors.external_port = 'Port must be between 1 and 65535';
    }

    // If external IP is provided, external port is required and vice versa
    if (formData.external_ip && !formData.external_port) {
      newErrors.external_port = 'Port is required when IP address is specified';
    }
    if (formData.external_port && !formData.external_ip) {
      newErrors.external_ip = 'IP address is required when port is specified';
    }

    // Validate local port if provided
    if (formData.port && !validatePort(formData.port)) {
      newErrors.port = 'Port must be between 1 and 65535';
    }

    // Validate local IP if provided
    if (formData.ipAddress && !validateIpAddress(formData.ipAddress)) {
      newErrors.ipAddress = 'Please enter a valid IP address (e.g., 192.168.1.100)';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleInputChange = (field: keyof Analyzer, value: string | number | boolean) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));

    // Clear error for this field when user starts typing
    if (errors[field]) {
      setErrors(prev => {
        const newErrors = { ...prev };
        delete newErrors[field];
        return newErrors;
      });
    }
  };

  const handleSave = async () => {
    if (!validateForm()) {
      return;
    }

    try {
      await onSave(formData);
      onOpenChange(false);
    } catch (error) {
      console.error('Failed to save analyzer configuration:', error);
    }
  };

  const handleClose = () => {
    setErrors({});
    onOpenChange(false);
  };

  if (!analyzer) {
    return null;
  }

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[600px] max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle className="flex items-center space-x-2">
            <Network className="h-5 w-5" />
            <span>Configure {analyzer.name}</span>
          </DialogTitle>
          <DialogDescription>
            Configure the connection settings for this analyzer
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          {/* Basic Information */}
          <div className="space-y-4">
            <h3 className="text-sm font-medium text-muted-foreground">Basic Information</h3>
            
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="name">Analyzer Name</Label>
                <Input
                  id="name"
                  placeholder="Enter analyzer name"
                  value={formData.name || ''}
                  onChange={(e) => handleInputChange('name', e.target.value)}
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="model">Model</Label>
                <Input
                  id="model"
                  placeholder="Enter model"
                  value={formData.model || ''}
                  onChange={(e) => handleInputChange('model', e.target.value)}
                />
              </div>
            </div>

            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="manufacturer">Manufacturer</Label>
                <Input
                  id="manufacturer"
                  placeholder="Enter manufacturer"
                  value={formData.manufacturer || ''}
                  onChange={(e) => handleInputChange('manufacturer', e.target.value)}
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="serialNumber">Serial Number</Label>
                <Input
                  id="serialNumber"
                  placeholder="Enter serial number"
                  value={formData.serialNumber || ''}
                  onChange={(e) => handleInputChange('serialNumber', e.target.value)}
                />
              </div>
            </div>
          </div>

          <Separator />

          {/* Local Connection Settings */}
          <div className="space-y-4">
            <div className="flex items-center space-x-2">
              <Server className="h-4 w-4 text-muted-foreground" />
              <h3 className="text-sm font-medium text-muted-foreground">Local Listener Settings</h3>
            </div>
            <p className="text-xs text-muted-foreground">
              Configure where this LIS server will listen for incoming connections from the analyzer
            </p>
            
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="ipAddress">Local IP Address</Label>
                <Input
                  id="ipAddress"
                  placeholder="e.g., 0.0.0.0 or 192.168.1.100"
                  value={formData.ipAddress || ''}
                  onChange={(e) => handleInputChange('ipAddress', e.target.value)}
                />
                {errors.ipAddress && (
                  <Alert variant="destructive" className="py-2">
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription className="text-xs">{errors.ipAddress}</AlertDescription>
                  </Alert>
                )}
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="port">Local Port</Label>
                <Input
                  id="port"
                  type="number"
                  placeholder="e.g., 9001"
                  min="1"
                  max="65535"
                  value={formData.port || ''}
                  onChange={(e) => handleInputChange('port', parseInt(e.target.value) || 0)}
                />
                {errors.port && (
                  <Alert variant="destructive" className="py-2">
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription className="text-xs">{errors.port}</AlertDescription>
                  </Alert>
                )}
              </div>
            </div>
          </div>

          <Separator />

          {/* External Connection Settings */}
          <div className="space-y-4">
            <div className="flex items-center space-x-2">
              <Network className="h-4 w-4 text-muted-foreground" />
              <h3 className="text-sm font-medium text-muted-foreground">External LIS Connection</h3>
            </div>
            <p className="text-xs text-muted-foreground">
              Configure where to send messages to external LIS instruments (optional)
            </p>
            
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="external_ip">External LIS IP Address</Label>
                <Input
                  id="external_ip"
                  placeholder="e.g., 192.168.1.200"
                  value={formData.external_ip || ''}
                  onChange={(e) => handleInputChange('external_ip', e.target.value)}
                />
                {errors.external_ip && (
                  <Alert variant="destructive" className="py-2">
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription className="text-xs">{errors.external_ip}</AlertDescription>
                  </Alert>
                )}
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="external_port">External LIS Port</Label>
                <Input
                  id="external_port"
                  type="number"
                  placeholder="e.g., 9002"
                  min="1"
                  max="65535"
                  value={formData.external_port || ''}
                  onChange={(e) => handleInputChange('external_port', parseInt(e.target.value) || 0)}
                />
                {errors.external_port && (
                  <Alert variant="destructive" className="py-2">
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription className="text-xs">{errors.external_port}</AlertDescription>
                  </Alert>
                )}
              </div>
            </div>
            
            {formData.external_ip && formData.external_port && (
              <div className="p-3 bg-muted/50 rounded-md">
                <p className="text-xs text-muted-foreground">
                  <strong>External Connection:</strong> {formData.external_ip}:{formData.external_port}
                </p>
              </div>
            )}
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose} disabled={loading}>
            Cancel
          </Button>
          <Button onClick={handleSave} disabled={loading}>
            {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Save Configuration
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}