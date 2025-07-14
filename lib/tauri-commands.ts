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

// HL7 Settings type for BF-6500
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

// BF-6500 specific response types
export interface BF6500ConfigResponse {
  success: boolean;
  analyzer?: AnalyzerResponse;
  hl7_settings?: HL7Settings;
  error_message?: string;
}

export interface BF6500ServiceStatus {
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

// BF-6500 commands
export const fetchBF6500Config = async (): Promise<BF6500ConfigResponse> => {
  return invoke('fetch_bf6500_config');
};

export const updateBF6500Config = async (
  analyzer: any,
  hl7Settings: HL7Settings
): Promise<BF6500ConfigResponse> => {
  return invoke('update_bf6500_config', { analyzer, hl7_settings: hl7Settings });
};

export const getBF6500ServiceStatus = async (): Promise<BF6500ServiceStatus> => {
  return invoke('get_bf6500_service_status');
};

export const startBF6500Service = async (): Promise<void> => {
  return invoke('start_bf6500_service');
};

export const stopBF6500Service = async (): Promise<void> => {
  return invoke('stop_bf6500_service');
}; 