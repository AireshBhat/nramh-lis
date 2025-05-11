use crate::protocol::constants::{
    COMMENT_RECORD, COMPONENT_DELIMITER, ESCAPE_DELIMITER, FIELD_DELIMITER, HEADER_RECORD,
    ORDER_RECORD, PATIENT_RECORD, REPEAT_DELIMITER, REQUEST_RECORD, RESULT_RECORD,
    TERMINATOR_RECORD,
};
use crate::protocol::error::ProtocolError;
use std::collections::HashMap;

/// The type of ASTM record
#[derive(Debug, Clone, PartialEq)]
pub enum RecordType {
    Header,
    Patient,
    Order,
    Result,
    Comment,
    Request,
    Terminator,
}

impl RecordType {
    /// Parse a record type from its identifier
    pub fn from_identifier(id: &str) -> Option<Self> {
        match id {
            "H" => Some(RecordType::Header),
            "P" => Some(RecordType::Patient),
            "O" => Some(RecordType::Order),
            "R" => Some(RecordType::Result),
            "C" => Some(RecordType::Comment),
            "Q" => Some(RecordType::Request),
            "L" => Some(RecordType::Terminator),
            _ => None,
        }
    }

    /// Get the identifier for this record type
    pub fn to_identifier(&self) -> &'static str {
        match self {
            RecordType::Header => HEADER_RECORD,
            RecordType::Patient => PATIENT_RECORD,
            RecordType::Order => ORDER_RECORD,
            RecordType::Result => RESULT_RECORD,
            RecordType::Comment => COMMENT_RECORD,
            RecordType::Request => REQUEST_RECORD,
            RecordType::Terminator => TERMINATOR_RECORD,
        }
    }
}

/// Represents an ASTM record at the application layer
#[derive(Debug, Clone)]
pub struct Record {
    /// The type of record
    pub record_type: RecordType,
    /// The fields of the record
    pub fields: HashMap<usize, String>,
}

impl Record {
    /// Create a new record with the given type
    pub fn new(record_type: RecordType) -> Self {
        Self {
            record_type,
            fields: HashMap::new(),
        }
    }

    /// Set a field value
    pub fn set_field(&mut self, index: usize, value: String) {
        self.fields.insert(index, value);
    }

    /// Get a field value
    pub fn get_field(&self, index: usize) -> Option<&String> {
        self.fields.get(&index)
    }

    /// Parse a record from a string
    pub fn parse(data: &str) -> Result<Self, ProtocolError> {
        if data.is_empty() {
            return Err(ProtocolError::InvalidRecordFormat(
                "Empty record".to_string(),
            ));
        }

        // Extract record type identifier
        let record_type_char = &data[0..1];
        let record_type = RecordType::from_identifier(record_type_char).ok_or_else(|| {
            ProtocolError::InvalidRecordFormat(format!("Unknown record type: {}", record_type_char))
        })?;

        let mut record = Record::new(record_type);

        // Split the record into fields by the field delimiter
        let fields: Vec<&str> = data.split(FIELD_DELIMITER as char).collect();

        // Process each field
        for (i, field) in fields.iter().enumerate() {
            if !field.is_empty() || i == 0 {
                record.set_field(i, field.to_string());
            }
        }

        Ok(record)
    }

    /// Encode the record to a string
    pub fn encode(&self) -> String {
        let mut result = String::new();

        // Start with the record type identifier
        result.push_str(self.record_type.to_identifier());

        // Find the highest field index
        let max_index = self.fields.keys().max().unwrap_or(&0);

        // Add each field
        for i in 1..=*max_index {
            result.push(FIELD_DELIMITER as char);
            if let Some(field) = self.fields.get(&i) {
                result.push_str(field);
            }
        }

        result
    }
}
