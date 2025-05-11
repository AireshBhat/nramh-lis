use crate::protocol::constants::{CR, ETB, ETX, LF, STX};
use crate::protocol::error::ProtocolError;

/// Represents an ASTM frame at the data link layer
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
    pub fn parse(data: &[u8]) -> Result<Self, ProtocolError> {
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
    fn parse_checksum(checksum_bytes: &[u8]) -> Result<u8, ProtocolError> {
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
