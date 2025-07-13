use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpAddressResponse {
    pub ip_address: String,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Fetches the local IP address of the system
pub fn get_local_ip_address() -> IpAddressResponse {
    match local_ip() {
        Ok(ip) => {
            log::info!("Local IP address retrieved successfully: {}", ip);
            IpAddressResponse {
                ip_address: ip.to_string(),
                success: true,
                error_message: None,
            }
        }
        Err(e) => {
            log::error!("Failed to get local IP address: {}", e);
            IpAddressResponse {
                ip_address: String::new(),
                success: false,
                error_message: Some(format!("Failed to get local IP address: {}", e)),
            }
        }
    }
}

/// Tauri command that returns just the IP address string
#[tauri::command]
pub fn get_local_ip() -> Result<String, String> {
    local_ip()
        .map(|ip| ip.to_string())
        .map_err(|e| format!("Failed to get local IP address: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip_address() {
        let result = get_local_ip_address();
        assert!(result.success || result.error_message.is_some());

        if result.success {
            assert!(!result.ip_address.is_empty());
            println!("Local IP: {}", result.ip_address);
        } else {
            println!("Error: {:?}", result.error_message);
        }
    }
}
