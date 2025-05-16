// ASTM Protocol Constants based on the specifications

// Transmission characters
pub const STX: u8 = 0x02; // Start of Text
pub const ETX: u8 = 0x03; // End of Text
pub const EOT: u8 = 0x04; // End of Transmission
pub const ENQ: u8 = 0x05; // Enquiry
pub const ACK: u8 = 0x06; // Acknowledgment
pub const NAK: u8 = 0x15; // Negative Acknowledgment
pub const ETB: u8 = 0x17; // End of Transmission Block
pub const CR: u8 = 0x0D; // Carriage Return
pub const LF: u8 = 0x0A; // Line Feed

// Delimiters
pub const FIELD_DELIMITER: u8 = b'|';
pub const REPEAT_DELIMITER: u8 = b'`';
pub const COMPONENT_DELIMITER: u8 = b'^';
pub const ESCAPE_DELIMITER: u8 = b'&';

// Record types
pub const HEADER_RECORD: &str = "H";
pub const PATIENT_RECORD: &str = "P";
pub const ORDER_RECORD: &str = "O";
pub const RESULT_RECORD: &str = "R";
pub const COMMENT_RECORD: &str = "C";
pub const REQUEST_RECORD: &str = "Q";
pub const TERMINATOR_RECORD: &str = "L";

// Timeouts (in milliseconds)
pub const CONNECT_TIMEOUT: u64 = 5000;
pub const READ_TIMEOUT: u64 = 10000;
pub const WRITE_TIMEOUT: u64 = 5000;
