use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::models::hematology::HematologyResult;
use crate::services::autoquant_meril::TestResult;

// ============================================================================
// HIS API DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HisTestValue {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HisApiPayload {
    #[serde(rename = "Machine")]
    pub machine: String,
    #[serde(rename = "SentOn")]
    pub sent_on: String,
    #[serde(rename = "SampleNo")]
    pub sample_no: String,
    #[serde(rename = "Sent")]
    pub sent: bool,
    #[serde(rename = "Values")]
    pub values: Vec<HisTestValue>,
}

#[derive(Debug, Clone)]
pub struct HisApiConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub retry_delay_seconds: u64,
}

impl Default for HisApiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://192.168.1.99/caremap/machine_interface/machine_data_ayush".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
            retry_delay_seconds: 5,
        }
    }
}

// ============================================================================
// HIS API CLIENT
// ============================================================================

pub struct HisClient {
    config: HisApiConfig,
    client: reqwest::Client,
}

impl HisClient {
    pub fn new(config: HisApiConfig) -> Self {
        log::debug!("Creating HIS client with config: {:?}", config);
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap();

        log::info!("HIS client initialized with timeout: {}s, retry attempts: {}, retry delay: {}s", 
                   config.timeout_seconds, config.retry_attempts, config.retry_delay_seconds);

        Self { config, client }
    }

    pub fn with_default_config() -> Self {
        log::debug!("Creating HIS client with default configuration");
        Self::new(HisApiConfig::default())
    }

    /// Send lab results from AutoQuant Meril analyzer to HIS system
    pub async fn send_meril_results(
        &self,
        analyzer_id: &str,
        patient_id: Option<&str>,
        test_results: &[TestResult],
    ) -> Result<(), String> {
        log::info!("Starting to send Meril results - Analyzer: {}, Patient: {:?}, Test count: {}", 
                   analyzer_id, patient_id, test_results.len());
        
        log::debug!("Meril test results details: {:?}", test_results);
        
        let machine_name = "Meril-3.6-11052213".to_string();
        let sample_no = patient_id.unwrap_or("UNKNOWN").to_string();
        
        log::debug!("Mapped analyzer '{}' to machine name '{}'", analyzer_id, machine_name);
        log::debug!("Using sample number: '{}'", sample_no);
        
        let values: Vec<HisTestValue> = test_results
            .iter()
            .map(|result| {
                let mapped_name = self.map_test_name(&result.sample_id);
                log::debug!("Mapping test ID '{}' to name '{}' with value '{}'", 
                           result.sample_id, mapped_name, result.value);
                HisTestValue {
                    name: mapped_name,
                    value: result.value.clone(),
                }
            })
            .collect();

        log::debug!("Constructed {} HIS test values", values.len());

        let payload = HisApiPayload {
            machine: machine_name,
            sent_on: Local::now().to_rfc3339(),
            sample_no,
            sent: true,
            values,
        };

        log::debug!("Constructed HIS API payload: {:?}", payload);
        log::info!("Sending Meril payload to HIS system for sample {}", payload.sample_no);

        self.send_payload(&payload).await
    }

    /// Send hematology results from BF-6900 analyzer to HIS system
    pub async fn send_hematology_results(
        &self,
        analyzer_id: &str,
        patient_id: Option<&str>,
        test_results: &[HematologyResult],
        timestamp: DateTime<Utc>,
    ) -> Result<(), String> {
        log::info!("Starting to send Hematology results - Analyzer: {}, Patient: {:?}, Test count: {}", 
                   analyzer_id, patient_id, test_results.len());
        
        log::debug!("Hematology test results details: {:?}", test_results);
        
        let machine_name = "Meril CQ 5 Plus".to_string();
        let sample_no = patient_id.unwrap_or("UNKNOWN").to_string();
        
        log::debug!("Mapped analyzer '{}' to machine name '{}'", analyzer_id, machine_name);
        log::debug!("Using sample number: '{}'", sample_no);
        
        let values: Vec<HisTestValue> = test_results
            .iter()
            .map(|result| {
                log::debug!("Processing hematology parameter '{}' with value '{}'", 
                           result.parameter, result.value);
                HisTestValue {
                    name: result.parameter.clone(),
                    value: result.value.clone(),
                }
            })
            .collect();

        log::debug!("Constructed {} HIS test values", values.len());

        let payload = HisApiPayload {
            machine: machine_name,
            sent_on: Local::now().to_rfc3339(),
            sample_no,
            sent: true,
            values,
        };

        log::debug!("Constructed HIS API payload: {:?}", payload);
        log::info!("Sending Hematology payload to HIS system for sample {}", payload.sample_no);

        self.send_payload(&payload).await
    }

    /// Send the payload to HIS system with retry logic
    async fn send_payload(&self, payload: &HisApiPayload) -> Result<(), String> {
        log::debug!("Starting payload transmission to HIS system at URL: {}", self.config.base_url);
        log::debug!("Payload details - Machine: {}, Sample: {}, Values count: {}", 
                   payload.machine, payload.sample_no, payload.values.len());
        
        let mut last_error = String::new();
        
        for attempt in 0..self.config.retry_attempts {
            log::debug!("Attempt {} of {} to send payload to HIS system", 
                       attempt + 1, self.config.retry_attempts);
            
            match self.send_request(payload).await {
                Ok(_) => {
                    log::info!(
                        "Successfully sent data to HIS system for sample {} (attempt {})",
                        payload.sample_no,
                        attempt + 1
                    );
                    log::debug!("Payload transmission completed successfully");
                    return Ok(());
                }
                Err(e) => {
                    last_error = e;
                    log::warn!(
                        "Failed to send data to HIS system for sample {} (attempt {}): {}",
                        payload.sample_no,
                        attempt + 1,
                        last_error
                    );
                    
                    if attempt < self.config.retry_attempts - 1 {
                        log::debug!("Waiting {} seconds before retry attempt {}", 
                                   self.config.retry_delay_seconds, attempt + 2);
                        tokio::time::sleep(Duration::from_secs(self.config.retry_delay_seconds)).await;
                    } else {
                        log::error!("All {} retry attempts exhausted for sample {}", 
                                   self.config.retry_attempts, payload.sample_no);
                    }
                }
            }
        }

        let error_msg = format!(
            "Failed to send data to HIS system after {} attempts: {}",
            self.config.retry_attempts, last_error
        );
        log::error!("{}", error_msg);
        Err(error_msg)
    }

    /// Send a single HTTP request to HIS system
    async fn send_request(&self, payload: &HisApiPayload) -> Result<(), String> {
        log::debug!("Preparing HTTP POST request to: {}", self.config.base_url);
        log::debug!("Request payload JSON: {}", serde_json::to_string_pretty(payload).unwrap_or_default());
        
        let start_time = std::time::Instant::now();
        
        let response = match self
            .client
            .post(&self.config.base_url)
            .json(payload)
            .send()
            .await
        {
            Ok(response) => {
                let duration = start_time.elapsed();
                log::debug!("HTTP request completed in {:?} with status: {}", duration, response.status());
                response
            }
            Err(e) => {
                let duration = start_time.elapsed();
                log::error!("HTTP request failed after {:?}: {}", duration, e);
                return Err(format!("HTTP request failed: {}", e));
            }
        };

        if response.status().is_success() {
            log::debug!("HIS API response successful: {:?}", response.status());
            
            // Log response headers for debugging
            log::debug!("Response headers: {:?}", response.headers());
            
            // Try to read and log response body for debugging
            match response.text().await {
                Ok(body) => {
                    if !body.is_empty() {
                        log::debug!("HIS API response body: {}", body);
                    } else {
                        log::debug!("HIS API response body is empty");
                    }
                }
                Err(e) => {
                    log::warn!("Failed to read response body: {}", e);
                }
            }
            
            Ok(())
        } else {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            log::error!("HIS API returned error status {}: {}", status, body);
            
            Err(format!(
                "HIS API returned error status {}: {}",
                status, body
            ))
        }
    }

    /// Map analyzer ID to machine name for HIS system
    fn get_machine_name_for_analyzer(&self, analyzer_id: &str) -> String {
        log::debug!("Mapping analyzer ID '{}' to machine name", analyzer_id);
        
        let machine_name = if analyzer_id.contains("bf6900") || analyzer_id.contains("hematology") {
            "Meril CQ 5 Plus".to_string()
        } else if analyzer_id.contains("autoquant") || analyzer_id.contains("meril") {
            "Meril-3.6-11052213".to_string()
        } else {
            // Default fallback
            "Unknown-Analyzer".to_string()
        };
        
        log::debug!("Mapped analyzer '{}' to machine name '{}'", analyzer_id, machine_name);
        machine_name
    }

    /// Map internal test IDs to HIS system test names
    fn map_test_name(&self, test_id: &str) -> String {
        log::debug!("Mapping test ID '{}' to HIS test name", test_id);
        
        // Remove ASTM formatting and return clean test name
        let clean_name = test_id.replace("^^^", "").replace("^^", "");
        log::debug!("Cleaned test ID '{}' to '{}'", test_id, clean_name);
        
        // Map common test names to HIS expected format
        let mapped_name = match clean_name.to_uppercase().as_str() {
            "ALB" => "ALB".to_string(),
            "AST" => "AST".to_string(),
            "ALT" => "ALT".to_string(),
            "GLU" | "GLUC" | "GLU-G" => "Glu-G".to_string(),
            "CREA" | "CREAT" | "CREA-S" => "CREA-S".to_string(),
            "TG" | "TRIG" => "TG".to_string(),
            "HDL" | "HDL-C" => "HDL-C".to_string(),
            "TC" | "CHOL" => "TC".to_string(),
            "UREA" | "BUN" => "UREA".to_string(),
            _ => clean_name,
        };
        
        log::debug!("Mapped test ID '{}' to HIS name '{}'", test_id, mapped_name);
        mapped_name
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_his_api_payload_serialization() {
        let payload = HisApiPayload {
            machine: "Mindray-BS-430".to_string(),
            sent_on: "2024-07-04T10:46:43.2170383+05:30".to_string(),
            sample_no: "117217".to_string(),
            sent: true,
            values: vec![
                HisTestValue {
                    name: "AST".to_string(),
                    value: "17.36".to_string(),
                },
                HisTestValue {
                    name: "ALT".to_string(),
                    value: "15.05".to_string(),
                },
            ],
        };

        let json = serde_json::to_string_pretty(&payload).unwrap();
        println!("HIS API Payload JSON:\n{}", json);
        
        // Verify the structure matches expected format
        assert!(json.contains("\"Machine\""));
        assert!(json.contains("\"SentOn\""));
        assert!(json.contains("\"SampleNo\""));
        assert!(json.contains("\"Sent\""));
        assert!(json.contains("\"Values\""));
    }

    #[test]
    fn test_machine_name_mapping() {
        let client = HisClient::with_default_config();
        
        assert_eq!(
            client.get_machine_name_for_analyzer("bf6900-001"),
            "Meril-BF-6900"
        );
        assert_eq!(
            client.get_machine_name_for_analyzer("autoquant-meril-001"),
            "Mindray-BS-430"
        );
        assert_eq!(
            client.get_machine_name_for_analyzer("unknown-device"),
            "Unknown-Analyzer"
        );
    }

    #[test]
    fn test_test_name_mapping() {
        let client = HisClient::with_default_config();
        
        assert_eq!(client.map_test_name("^^^AST"), "AST");
        assert_eq!(client.map_test_name("^^^ALT"), "ALT");
        assert_eq!(client.map_test_name("^^^GLU"), "Glu-G");
        assert_eq!(client.map_test_name("^^^CREA"), "CREA-S");
        assert_eq!(client.map_test_name("CUSTOM_TEST"), "CUSTOM_TEST");
    }

    #[tokio::test]
    async fn test_his_client_creation() {
        let client = HisClient::with_default_config();
        assert_eq!(client.config.base_url, "http://192.168.1.99/caremap/machine_interface/machine_data_ayush");
        assert_eq!(client.config.timeout_seconds, 30);
        assert_eq!(client.config.retry_attempts, 3);
    }
}