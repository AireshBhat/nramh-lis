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

// HL7 Settings type for BF-6900
export interface HL7Settings {
  timeout_ms: number;
  retry_attempts: number;
  encoding: string;
  supported_message_types: string[];
}

// Meril-specific response types
export interface MerilConfigResponse {
  success: boolean;
  analyzer?: AnalyzerResponse;
  error_message?: string;
}

// BF-6900 specific response types
export interface BF6900ConfigResponse {
  success: boolean;
  analyzer?: AnalyzerResponse;
  hl7_settings?: HL7Settings;
  error_message?: string;
}

export interface BF6900ServiceStatus {
  is_running: boolean;
  connections_count: number;
  analyzer_status: string;
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

// Meril commands
export const fetchMerilConfig = async (): Promise<MerilConfigResponse> => {
  return invoke('fetch_meril_config');
};

export const startMerilService = async (): Promise<void> => {
  return invoke('start_meril_service');
};

export const stopMerilService = async (): Promise<void> => {
  return invoke('stop_meril_service');
};

// BF-6900 commands
export const fetchBF6900Config = async (): Promise<BF6900ConfigResponse> => {
  return invoke('fetch_bf6900_config');
};

export const updateBF6900Config = async (
  analyzer: any,
  hl7Settings: HL7Settings
): Promise<BF6900ConfigResponse> => {
  return invoke('update_bf6900_config', { analyzer, hl7_settings: hl7Settings });
};

export const getBF6900ServiceStatus = async (): Promise<BF6900ServiceStatus> => {
  return invoke('get_bf6900_service_status');
};

export const startBF6900Service = async (): Promise<void> => {
  return invoke('start_bf6900_service');
};

export const stopBF6900Service = async (): Promise<void> => {
  return invoke('stop_bf6900_service');
}; 