use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;

// Model imports with correct paths
use crate::model::{Patient, TestOrder, TestResult};

pub mod constants;

use crate::protocol::error::{ProtocolError, Result};
use constants::{
    COMMENT_RECORD, COMPONENT_DELIMITER, CR, ESCAPE_DELIMITER, ETB, ETX, FIELD_DELIMITER,
    HEADER_RECORD, LF, ORDER_RECORD, PATIENT_RECORD, REPEAT_DELIMITER, REQUEST_RECORD,
    RESULT_RECORD, STX, TERMINATOR_RECORD,
};

/// Represents a Record Type in the ASTM protocol
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

/// Represents an ASTM Record with fields
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
    pub fn parse(data: &str) -> Result<Self> {
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
            record.set_field(i, field.to_string());
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

/// Represents an ASTM Frame at the data link layer
#[derive(Debug, Clone)]
pub struct Frame {
    /// The sequence number of the frame
    pub sequence: u8,
    /// The raw content of the frame (without STX, ETX/ETB, and checksum)
    pub content: Vec<u8>,
    /// Whether this is the last frame in a message
    pub is_last_frame: bool,
}

impl Frame {
    /// Create a new frame with the given sequence number and content
    pub fn new(sequence: u8, content: Vec<u8>, is_last_frame: bool) -> Self {
        Self {
            sequence,
            content,
            is_last_frame,
        }
    }

    /// Encode the frame to bytes according to ASTM protocol
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // Start of frame
        buffer.push(STX);

        // Frame content
        buffer.push(self.sequence + b'0'); // Convert to ASCII
        buffer.extend_from_slice(&self.content);

        // End of frame marker
        if self.is_last_frame {
            buffer.push(ETX);
        } else {
            buffer.push(ETB);
        }

        // Calculate checksum
        let checksum = Self::calculate_checksum(&buffer[1..]);

        // Add checksum
        let checksum_bytes = format!("{:02X}", checksum).into_bytes();
        buffer.extend_from_slice(&checksum_bytes);

        // Add CR LF
        buffer.push(CR);
        buffer.push(LF);

        buffer
    }

    /// Parse a frame from bytes according to ASTM protocol
    pub fn parse(data: &[u8]) -> Result<Self> {
        // Validate minimum frame length
        if data.len() < 7 {
            return Err(ProtocolError::InvalidFrameFormat(
                "Frame too short".to_string(),
            ));
        }

        // Validate STX
        if data[0] != STX {
            return Err(ProtocolError::InvalidFrameFormat(format!(
                "Invalid start byte: {:02X}",
                data[0]
            )));
        }

        // Find end of frame
        let end_position = data
            .iter()
            .position(|&b| b == ETX || b == ETB)
            .ok_or_else(|| ProtocolError::InvalidFrameFormat("Missing ETX/ETB".to_string()))?;

        let is_last_frame = data[end_position] == ETX;

        // Parse sequence number
        let sequence = data[1].checked_sub(b'0').ok_or_else(|| {
            ProtocolError::InvalidFrameFormat("Invalid sequence number".to_string())
        })?;

        // Extract content (excluding sequence number)
        let content = data[2..end_position].to_vec();

        // Validate checksum
        if data.len() < end_position + 3 {
            return Err(ProtocolError::InvalidFrameFormat(
                "Missing checksum".to_string(),
            ));
        }

        let expected_checksum = Self::parse_checksum(&data[end_position + 1..end_position + 3])?;
        let calculated_checksum = Self::calculate_checksum(&data[1..=end_position]);

        if expected_checksum != calculated_checksum {
            return Err(ProtocolError::InvalidChecksum {
                expected: format!("{:02X}", expected_checksum),
                actual: format!("{:02X}", calculated_checksum),
            });
        }

        Ok(Self {
            sequence,
            content,
            is_last_frame,
        })
    }

    /// Calculate the checksum of a byte slice (modulus 8 of the sum of ASCII values)
    fn calculate_checksum(data: &[u8]) -> u8 {
        let sum: u16 = data.iter().map(|&b| b as u16).sum();
        (sum % 256) as u8
    }

    /// Parse a two-character hexadecimal string into a byte
    fn parse_checksum(checksum_bytes: &[u8]) -> Result<u8> {
        if checksum_bytes.len() < 2 {
            return Err(ProtocolError::InvalidFrameFormat(
                "Checksum too short".to_string(),
            ));
        }

        let hex_str = std::str::from_utf8(checksum_bytes).map_err(|_| {
            ProtocolError::InvalidFrameFormat("Invalid checksum encoding".to_string())
        })?;

        u8::from_str_radix(hex_str, 16)
            .map_err(|_| ProtocolError::InvalidFrameFormat("Invalid checksum format".to_string()))
    }
}

/// ASTM Protocol trait for encoding and decoding ASTM messages
pub trait AstmProtocol {
    /// Parse a date time string in ASTM format (YYYYMMDDHHMMSS)
    fn parse_datetime(dt_str: &str) -> Option<DateTime<Utc>> {
        if dt_str.len() < 8 {
            return None;
        }

        // Parse at least YYYYMMDD
        let year = dt_str[0..4].parse::<i32>().ok()?;
        let month = dt_str[4..6].parse::<u32>().ok()?;
        let day = dt_str[6..8].parse::<u32>().ok()?;

        // Default time components
        let mut hour = 0;
        let mut min = 0;
        let mut sec = 0;

        // Parse optional time components
        if dt_str.len() >= 10 {
            hour = dt_str[8..10].parse::<u32>().unwrap_or(0);
        }
        if dt_str.len() >= 12 {
            min = dt_str[10..12].parse::<u32>().unwrap_or(0);
        }
        if dt_str.len() >= 14 {
            sec = dt_str[12..14].parse::<u32>().unwrap_or(0);
        }

        let datetime = NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(year, month, day)?,
            chrono::NaiveTime::from_hms_opt(hour, min, sec)?,
        );

        Some(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    }

    /// Format a datetime to ASTM format string
    fn format_datetime(dt: &DateTime<Utc>) -> String {
        dt.format("%Y%m%d%H%M%S").to_string()
    }

    /// Parse a patient record into a Patient model
    fn parse_patient_record(record: &Record) -> Result<Patient>;

    /// Parse a test order record into a TestOrder model
    fn parse_test_order_record(record: &Record) -> Result<TestOrder>;

    /// Parse a result record into a TestResult model
    fn parse_result_record(record: &Record) -> Result<TestResult>;

    /// Encode a Patient model into a patient record
    fn encode_patient_record(patient: &Patient) -> Result<Record>;

    /// Encode a TestOrder model into a test order record
    fn encode_test_order_record(order: &TestOrder) -> Result<Record>;

    /// Encode a TestResult model into a result record
    fn encode_result_record(result: &TestResult) -> Result<Record>;

    /// Create a header record
    fn create_header_record() -> Record {
        let mut record = Record::new(RecordType::Header);
        record.set_field(
            1,
            format!(
                "{}{}{}{}",
                FIELD_DELIMITER as char,
                REPEAT_DELIMITER as char,
                COMPONENT_DELIMITER as char,
                ESCAPE_DELIMITER as char
            ),
        );
        record.set_field(12, "P".to_string()); // Processing ID: Production
        record.set_field(13, "E 1394-97".to_string()); // ASTM version
        record.set_field(14, Self::format_datetime(&Utc::now())); // Current date/time
        record
    }

    /// Create a terminator record
    fn create_terminator_record() -> Record {
        let mut record = Record::new(RecordType::Terminator);
        record.set_field(1, "1".to_string()); // Sequence number
        record.set_field(2, "N".to_string()); // Normal termination
        record
    }

    /// Parse components from a component string (separated by component delimiter)
    fn parse_components(component_str: &str) -> Vec<String> {
        component_str
            .split(COMPONENT_DELIMITER as char)
            .map(|s| s.to_string())
            .collect()
    }

    /// Parse repeats from a repeat string (separated by repeat delimiter)
    fn parse_repeats(repeat_str: &str) -> Vec<String> {
        repeat_str
            .split(REPEAT_DELIMITER as char)
            .map(|s| s.to_string())
            .collect()
    }

    /// Join components into a component string
    fn join_components(components: &[String]) -> String {
        components.join(&(COMPONENT_DELIMITER as char).to_string())
    }

    /// Join repeats into a repeat string
    fn join_repeats(repeats: &[String]) -> String {
        repeats.join(&(REPEAT_DELIMITER as char).to_string())
    }

    /// Split a frame into records
    fn split_frame_to_records(frame: &Frame) -> Result<Vec<Record>> {
        let content = String::from_utf8(frame.content.clone())
            .map_err(|_| ProtocolError::InvalidFrameFormat("Invalid UTF-8".to_string()))?;

        let records = content
            .split(CR as char)
            .filter(|s| !s.is_empty())
            .map(Record::parse)
            .collect::<Result<Vec<Record>>>()?;

        Ok(records)
    }

    /// Join records into a frame content
    fn join_records_to_frame_content(records: &[Record]) -> Vec<u8> {
        let content = records
            .iter()
            .map(|r| r.encode() + &(CR as char).to_string())
            .collect::<Vec<String>>()
            .join("");

        content.into_bytes()
    }
}
