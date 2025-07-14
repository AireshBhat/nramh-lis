use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::result::{TestResult, TestResultMetadata, ReferenceRange, ResultFlags, ResultStatus};

// ============================================================================
// HL7 PATIENT DATA STRUCTURE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientData {
    pub id: String,
    pub name: String,
    pub birth_date: Option<String>,
    pub sex: Option<String>,
    pub address: Option<String>,
    pub telephone: Option<String>,
    pub physicians: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
}

// ============================================================================
// BF-6500 EVENT TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BF6500Event {
    /// Analyzer connected
    AnalyzerConnected {
        analyzer_id: String,
        remote_addr: String,
        timestamp: DateTime<Utc>,
    },
    /// Analyzer disconnected
    AnalyzerDisconnected {
        analyzer_id: String,
        timestamp: DateTime<Utc>,
    },
    /// HL7 message received
    HL7MessageReceived {
        analyzer_id: String,
        message_type: String,
        raw_data: String,
        timestamp: DateTime<Utc>,
    },
    /// Hematology result processed
    HematologyResultProcessed {
        analyzer_id: String,
        patient_id: Option<String>,
        patient_data: Option<PatientData>,
        test_results: Vec<HematologyResult>,
        timestamp: DateTime<Utc>,
    },
    /// Analyzer status updated
    AnalyzerStatusUpdated {
        analyzer_id: String,
        status: crate::models::AnalyzerStatus,
        timestamp: DateTime<Utc>,
    },
    /// Error occurred
    Error {
        analyzer_id: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
}

// ============================================================================
// HEMATOLOGY-SPECIFIC RESULT DATA
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematologyResult {
    pub id: String,
    pub parameter: String,           // WBC, RBC, HGB, HCT, MCV, MCH, MCHC, PLT
    pub parameter_code: String,      // Laboratory code for the parameter
    pub value: String,
    pub units: Option<String>,
    pub reference_range: Option<String>,
    pub flags: Vec<String>,          // H (High), L (Low), A (Abnormal), etc.
    pub status: String,              // F=Final, P=Preliminary, C=Correction
    pub completed_date_time: Option<DateTime<Utc>>,
    pub analyzer_id: Option<String>,
    pub sample_id: String,
    pub test_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<HematologyResult> for TestResult {
    fn from(hematology_result: HematologyResult) -> Self {
        // Parse reference range from string to ReferenceRange struct
        let reference_range = hematology_result.reference_range.and_then(|range_str| {
            // Parse range like "4.0-10.0" into lower and upper limits
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() == 2 {
                let lower = parts[0].parse::<f64>().ok();
                let upper = parts[1].parse::<f64>().ok();
                Some(ReferenceRange {
                    lower_limit: lower,
                    upper_limit: upper,
                })
            } else {
                None
            }
        });

        // Convert flags from Vec<String> to ResultFlags
        let flags = if !hematology_result.flags.is_empty() {
            Some(ResultFlags {
                abnormal_flag: hematology_result.flags.first().cloned(),
                nature_of_abnormality: hematology_result.flags.get(1).cloned(),
            })
        } else {
            None
        };

        // Convert status from String to ResultStatus
        let status = ResultStatus::from(hematology_result.status.as_str());

        TestResult {
            id: hematology_result.id,
            test_id: hematology_result.test_id,
            sample_id: hematology_result.sample_id,
            value: hematology_result.value,
            units: hematology_result.units,
            reference_range,
            flags,
            status,
            completed_date_time: hematology_result.completed_date_time,
            metadata: TestResultMetadata {
                sequence_number: 1, // Default sequence number
                instrument: hematology_result.analyzer_id.clone(),
            },
            analyzer_id: hematology_result.analyzer_id,
            created_at: hematology_result.created_at,
            updated_at: hematology_result.updated_at,
        }
    }
}

// ============================================================================
// HL7 CONFIGURATION SETTINGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HL7Settings {
    /// Enable MLLP framing
    pub mllp_enabled: bool,
    /// Connection timeout in milliseconds
    pub timeout_ms: u64,
    /// Number of retry attempts for failed operations
    pub retry_attempts: u32,
    /// Message encoding (typically UTF-8)
    pub encoding: String,
    /// Supported HL7 message types
    pub supported_message_types: Vec<String>,
    /// Application name for HL7 messages
    pub application_name: String,
    /// Facility name for HL7 messages
    pub facility_name: String,
    /// Auto-acknowledge messages
    pub auto_acknowledge: bool,
}

impl Default for HL7Settings {
    fn default() -> Self {
        Self {
            mllp_enabled: true,
            timeout_ms: 10000,
            retry_attempts: 3,
            encoding: "UTF-8".to_string(),
            supported_message_types: vec![
                "ORU^R01".to_string(), // Observation Result Unsolicited
                "OUL^R21".to_string(), // Unsolicited Laboratory Observation
            ],
            application_name: "BF6500_LIS".to_string(),
            facility_name: "HOSPITAL".to_string(),
            auto_acknowledge: true,
        }
    }
}

// ============================================================================
// HEMATOLOGY PARAMETER DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematologyParameter {
    pub code: String,
    pub name: String,
    pub display_name: String,
    pub units: String,
    pub reference_range_male: Option<String>,
    pub reference_range_female: Option<String>,
    pub reference_range_child: Option<String>,
    pub critical_low: Option<f64>,
    pub critical_high: Option<f64>,
}

/// Standard hematology parameters for BF-6500
pub fn get_standard_hematology_parameters() -> Vec<HematologyParameter> {
    vec![
        HematologyParameter {
            code: "WBC".to_string(),
            name: "White Blood Cells".to_string(),
            display_name: "WBC".to_string(),
            units: "10^9/L".to_string(),
            reference_range_male: Some("4.0-10.0".to_string()),
            reference_range_female: Some("4.0-10.0".to_string()),
            reference_range_child: Some("5.0-12.0".to_string()),
            critical_low: Some(2.0),
            critical_high: Some(20.0),
        },
        HematologyParameter {
            code: "RBC".to_string(),
            name: "Red Blood Cells".to_string(),
            display_name: "RBC".to_string(),
            units: "10^12/L".to_string(),
            reference_range_male: Some("4.5-5.5".to_string()),
            reference_range_female: Some("4.0-5.0".to_string()),
            reference_range_child: Some("4.0-5.2".to_string()),
            critical_low: Some(2.5),
            critical_high: Some(7.0),
        },
        HematologyParameter {
            code: "HGB".to_string(),
            name: "Hemoglobin".to_string(),
            display_name: "HGB".to_string(),
            units: "g/dL".to_string(),
            reference_range_male: Some("14.0-18.0".to_string()),
            reference_range_female: Some("12.0-16.0".to_string()),
            reference_range_child: Some("11.0-16.0".to_string()),
            critical_low: Some(7.0),
            critical_high: Some(20.0),
        },
        HematologyParameter {
            code: "HCT".to_string(),
            name: "Hematocrit".to_string(),
            display_name: "HCT".to_string(),
            units: "%".to_string(),
            reference_range_male: Some("42.0-52.0".to_string()),
            reference_range_female: Some("37.0-47.0".to_string()),
            reference_range_child: Some("34.0-44.0".to_string()),
            critical_low: Some(20.0),
            critical_high: Some(60.0),
        },
        HematologyParameter {
            code: "MCV".to_string(),
            name: "Mean Corpuscular Volume".to_string(),
            display_name: "MCV".to_string(),
            units: "fL".to_string(),
            reference_range_male: Some("80.0-100.0".to_string()),
            reference_range_female: Some("80.0-100.0".to_string()),
            reference_range_child: Some("75.0-95.0".to_string()),
            critical_low: Some(60.0),
            critical_high: Some(120.0),
        },
        HematologyParameter {
            code: "MCH".to_string(),
            name: "Mean Corpuscular Hemoglobin".to_string(),
            display_name: "MCH".to_string(),
            units: "pg".to_string(),
            reference_range_male: Some("27.0-32.0".to_string()),
            reference_range_female: Some("27.0-32.0".to_string()),
            reference_range_child: Some("25.0-33.0".to_string()),
            critical_low: Some(20.0),
            critical_high: Some(40.0),
        },
        HematologyParameter {
            code: "MCHC".to_string(),
            name: "Mean Corpuscular Hemoglobin Concentration".to_string(),
            display_name: "MCHC".to_string(),
            units: "g/dL".to_string(),
            reference_range_male: Some("32.0-36.0".to_string()),
            reference_range_female: Some("32.0-36.0".to_string()),
            reference_range_child: Some("32.0-36.0".to_string()),
            critical_low: Some(28.0),
            critical_high: Some(40.0),
        },
        HematologyParameter {
            code: "PLT".to_string(),
            name: "Platelets".to_string(),
            display_name: "PLT".to_string(),
            units: "10^9/L".to_string(),
            reference_range_male: Some("150-450".to_string()),
            reference_range_female: Some("150-450".to_string()),
            reference_range_child: Some("150-450".to_string()),
            critical_low: Some(50.0),
            critical_high: Some(1000.0),
        },
    ]
}

// ============================================================================
// BF-6500 CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BF6500Config {
    pub analyzer: crate::models::Analyzer,
    pub hl7_settings: HL7Settings,
    pub hematology_parameters: Vec<HematologyParameter>,
}

impl BF6500Config {
    pub fn new(analyzer: crate::models::Analyzer) -> Self {
        Self {
            analyzer,
            hl7_settings: HL7Settings::default(),
            hematology_parameters: get_standard_hematology_parameters(),
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Validates if a parameter code is a standard hematology parameter
pub fn is_hematology_parameter(parameter_code: &str) -> bool {
    let standard_params = get_standard_hematology_parameters();
    standard_params.iter().any(|p| p.code == parameter_code.to_uppercase())
}

/// Gets the display name for a hematology parameter
pub fn get_parameter_display_name(parameter_code: &str) -> String {
    let standard_params = get_standard_hematology_parameters();
    standard_params
        .iter()
        .find(|p| p.code == parameter_code.to_uppercase())
        .map(|p| p.display_name.clone())
        .unwrap_or_else(|| parameter_code.to_string())
}

/// Gets the reference range for a parameter based on gender
pub fn get_reference_range(parameter_code: &str, gender: Option<&str>) -> Option<String> {
    let standard_params = get_standard_hematology_parameters();
    let param = standard_params
        .iter()
        .find(|p| p.code == parameter_code.to_uppercase())?;
    
    match gender {
        Some("M") | Some("Male") => param.reference_range_male.clone(),
        Some("F") | Some("Female") => param.reference_range_female.clone(),
        Some("C") | Some("Child") => param.reference_range_child.clone(),
        _ => param.reference_range_male.clone(), // Default to male range
    }
}

/// Determines if a value is critical based on parameter thresholds
pub fn is_critical_value(parameter_code: &str, value: f64) -> bool {
    let standard_params = get_standard_hematology_parameters();
    if let Some(param) = standard_params.iter().find(|p| p.code == parameter_code.to_uppercase()) {
        if let Some(critical_low) = param.critical_low {
            if value < critical_low {
                return true;
            }
        }
        if let Some(critical_high) = param.critical_high {
            if value > critical_high {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hematology_parameter_validation() {
        assert!(is_hematology_parameter("WBC"));
        assert!(is_hematology_parameter("wbc")); // Case insensitive
        assert!(!is_hematology_parameter("INVALID"));
    }

    #[test]
    fn test_parameter_display_name() {
        assert_eq!(get_parameter_display_name("WBC"), "WBC");
        assert_eq!(get_parameter_display_name("HGB"), "HGB");
        assert_eq!(get_parameter_display_name("INVALID"), "INVALID");
    }

    #[test]
    fn test_reference_range_by_gender() {
        assert_eq!(
            get_reference_range("HGB", Some("M")),
            Some("14.0-18.0".to_string())
        );
        assert_eq!(
            get_reference_range("HGB", Some("F")),
            Some("12.0-16.0".to_string())
        );
        assert_eq!(
            get_reference_range("HGB", Some("C")),
            Some("11.0-16.0".to_string())
        );
    }

    #[test]
    fn test_critical_value_detection() {
        assert!(is_critical_value("HGB", 5.0)); // Below critical low
        assert!(is_critical_value("HGB", 25.0)); // Above critical high
        assert!(!is_critical_value("HGB", 15.0)); // Normal range
    }

    #[test]
    fn test_hl7_settings_default() {
        let settings = HL7Settings::default();
        assert!(settings.mllp_enabled);
        assert_eq!(settings.timeout_ms, 10000);
        assert_eq!(settings.retry_attempts, 3);
        assert!(settings.supported_message_types.contains(&"ORU^R01".to_string()));
    }

    #[test]
    fn test_hematology_result_to_test_result_conversion() {
        let hematology_result = HematologyResult {
            id: "test123".to_string(),
            parameter: "WBC".to_string(),
            parameter_code: "WBC".to_string(),
            value: "8.5".to_string(),
            units: Some("10^9/L".to_string()),
            reference_range: Some("4.0-10.0".to_string()),
            flags: vec!["N".to_string()],
            status: "F".to_string(),
            completed_date_time: Some(Utc::now()),
            analyzer_id: Some("bf6500-001".to_string()),
            sample_id: "S123".to_string(),
            test_id: "T123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let test_result: TestResult = hematology_result.into();
        assert_eq!(test_result.value, "8.5");
        assert_eq!(test_result.units, Some("10^9/L".to_string()));
    }
}