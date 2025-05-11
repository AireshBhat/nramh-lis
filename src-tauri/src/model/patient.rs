use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientName {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAddress {
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPhysicians {
    pub ordering: Option<String>,
    pub attending: Option<String>,
    pub referring: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalAttribute {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalAttributes {
    pub height: Option<PhysicalAttribute>,
    pub weight: Option<PhysicalAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    Other,
}

impl Display for Sex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Sex::Male => write!(f, "Male"),
            Sex::Female => write!(f, "Female"),
            Sex::Other => write!(f, "Other"),
        }
    }
}

impl From<&str> for Sex {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "M" => Sex::Male,
            "F" => Sex::Female,
            _ => Sex::Other,
        }
    }
}

impl From<Sex> for String {
    fn from(s: Sex) -> Self {
        match s {
            Sex::Male => "M".to_string(),
            Sex::Female => "F".to_string(),
            Sex::Other => "U".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,                        // Practice assigned patient ID (max 40 chars)
    pub name: PatientName,                 // Patient name components
    pub birth_date: Option<DateTime<Utc>>, // Format: YYYYMMDDHHMMSS
    pub sex: Sex,                          // M/F/U (Male/Female/Other)
    pub address: Option<PatientAddress>,   // Components separated by ^ in protocol
    pub telephone: Vec<String>,            // Multiple phone numbers
    pub physicians: Option<PatientPhysicians>, // From Attending Physician ID field
    pub physical_attributes: Option<PhysicalAttributes>, // Height and weight information
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
