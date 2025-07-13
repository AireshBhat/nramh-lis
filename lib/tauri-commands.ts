import { invoke } from '@tauri-apps/api/core';

// Types matching the Rust API responses
export interface AnalyzerResponse {
  id: string;
  name: string;
  model: string;
  serial_number?: string;
  manufacturer?: string;
  connection_type: string;
  ip_address?: string;
  port?: number;
  com_port?: string;
  baud_rate?: number;
  protocol: string;
  status: string;
  activate_on_start: boolean;
  created_at: string;
  updated_at: string;
}

// Meril-specific response types
export interface MerilConfigResponse {
  success: boolean;
  analyzer?: AnalyzerResponse;
  error_message?: string;
}

// Helper function to convert AnalyzerResponse to frontend Analyzer type
export function convertAnalyzerResponse(response: AnalyzerResponse) {
  return {
    id: response.id,
    name: response.name,
    model: response.model,
    serialNumber: response.serial_number,
    manufacturer: response.manufacturer,
    connectionType: { type: response.connection_type as 'Serial' | 'TcpIp' },
    ipAddress: response.ip_address,
    port: response.port,
    comPort: response.com_port,
    baudRate: response.baud_rate,
    protocol: { protocol: response.protocol as 'Astm' | 'Hl7' },
    status: { status: response.status as 'Active' | 'Inactive' | 'Maintenance' },
    activateOnStart: response.activate_on_start,
    createdAt: new Date(response.created_at),
    updatedAt: new Date(response.updated_at),
  };
}

// Simple Meril command
export const fetchMerilConfig = async (): Promise<MerilConfigResponse> => {
  return invoke('fetch_meril_config');
};

// Start Meril service command
export const startMerilService = async (): Promise<void> => {
  return invoke('start_meril_service');
};

// Stop Meril service command
export const stopMerilService = async (): Promise<void> => {
  return invoke('stop_meril_service');
}; 