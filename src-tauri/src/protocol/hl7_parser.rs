use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// MLLP PROTOCOL CONSTANTS
// ============================================================================

/// MLLP (Minimal Lower Layer Protocol) Start Block - Vertical Tab
pub const MLLP_START_BLOCK: u8 = 0x0B;

/// MLLP End Block - File Separator
pub const MLLP_END_BLOCK: u8 = 0x1C;

/// MLLP Carriage Return
pub const MLLP_CARRIAGE_RETURN: u8 = 0x0D;

// ============================================================================
// HL7 MESSAGE CONSTANTS
// ============================================================================

/// HL7 Field separator
pub const HL7_FIELD_SEPARATOR: char = '|';

/// HL7 Component separator
pub const HL7_COMPONENT_SEPARATOR: char = '^';

/// HL7 Repetition separator
pub const HL7_REPETITION_SEPARATOR: char = '~';

/// HL7 Escape character
pub const HL7_ESCAPE_CHARACTER: char = '\\';

/// HL7 Subcomponent separator
pub const HL7_SUBCOMPONENT_SEPARATOR: char = '&';

/// HL7 Segment separator
pub const HL7_SEGMENT_SEPARATOR: char = '\r';

// ============================================================================
// CQ 5 PLUS PARAMETER CODES (HL7 v2.3.1)
// ============================================================================

/// Creates mapping of CQ 5 Plus parameter codes to names
pub fn get_cq5_parameter_codes() -> HashMap<String, String> {
    let mut codes = HashMap::new();
    
    // Analysis modes and metadata
    codes.insert("2001".to_string(), "MODE".to_string()); // Analysis mode
    codes.insert("2002".to_string(), "MODE_EX".to_string()); // Measurement mode
    codes.insert("2003".to_string(), "Ref".to_string()); // Reference group
    codes.insert("2004".to_string(), "Note".to_string()); // Remarks
    codes.insert("2005".to_string(), "Level".to_string()); // QC level
    
    // Hematology parameters (CQ 5 Plus mapping)
    codes.insert("2006".to_string(), "V_WBC".to_string()); // Total white blood cell
    codes.insert("2007".to_string(), "V_NEU_p".to_string()); // Percentage of neutrophil
    codes.insert("2008".to_string(), "V_LYM_p".to_string()); // Percentage of lymphocyte
    codes.insert("2009".to_string(), "V_MON_p".to_string()); // Percentage of monocyte
    codes.insert("2010".to_string(), "V_EOS_p".to_string()); // Percentage of eosinophil
    codes.insert("2011".to_string(), "V_BAS_p".to_string()); // Percentage of basophil
    codes.insert("2012".to_string(), "V_NEU_c".to_string()); // Number of neutrophil
    codes.insert("2013".to_string(), "V_LYM_c".to_string()); // Number of lymphocyte
    codes.insert("2014".to_string(), "V_MON_c".to_string()); // Number of monocyte
    codes.insert("2015".to_string(), "V_EOS_c".to_string()); // Number of eosinophil
    codes.insert("2016".to_string(), "V_BAS_c".to_string()); // Number of basophil
    codes.insert("2017".to_string(), "V_RBC".to_string()); // Number of red blood cell
    codes.insert("2018".to_string(), "V_HGB".to_string()); // Hemoglobin
    codes.insert("2019".to_string(), "V_MCV".to_string()); // Mean red blood cell volume
    codes.insert("2020".to_string(), "V_HCT".to_string()); // RBC hematocrit
    codes.insert("2021".to_string(), "V_MCH".to_string()); // Mean red blood cell hemoglobin content
    codes.insert("2022".to_string(), "V_MCHC".to_string()); // Mean red blood cell hemoglobin concentration
    codes.insert("2023".to_string(), "V_RDW_SD".to_string()); // Standard deviation of red blood cell distribution width
    codes.insert("2024".to_string(), "V_RDW_CV".to_string()); // Red blood cell distribution width variation coefficient
    codes.insert("2025".to_string(), "V_PLT".to_string()); // Number of platelet
    codes.insert("2026".to_string(), "V_MPV".to_string()); // Average platelet volume
    codes.insert("2027".to_string(), "V_PCT".to_string()); // Platelet hematocrit
    codes.insert("2028".to_string(), "V_PDW".to_string()); // Platelet distribution width
    codes.insert("2029".to_string(), "V_P_LCR".to_string()); // Platelet - ratio of macrophage
    codes.insert("2030".to_string(), "V_P_LCC".to_string()); // Platelet ratio (NEW)
    codes.insert("2031".to_string(), "V_CRP".to_string()); // C reactive protein (NEW)
    codes.insert("2032".to_string(), "V_HS_CRP".to_string()); // Hypersensitive C-reactive protein (NEW)
    
    // Histogram/Scattergram data
    codes.insert("2101".to_string(), "RBCHistogram.PNG".to_string()); // RBC histogram PNG data
    codes.insert("2102".to_string(), "PLTHistogram.PNG".to_string()); // PLT histogram PNG data
    codes.insert("2033".to_string(), "BASOScattergram.PNG".to_string()); // BASO scattergram PNG data
    codes.insert("2034".to_string(), "DIFFScattergram.PNG".to_string()); // DIFF scattergram PNG data
    
    codes
}

/// Creates mapping of OBR-4 service type codes
pub fn get_obr4_service_codes() -> HashMap<String, String> {
    let mut codes = HashMap::new();
    
    codes.insert("1001".to_string(), "CountResults".to_string()); // Sample count results
    codes.insert("1002".to_string(), "LJQC".to_string()); // L-J QC count results
    codes.insert("1003".to_string(), "XbarQC".to_string()); // Xbar QC count results
    codes.insert("1004".to_string(), "XBQC".to_string()); // X-B QC count results
    codes.insert("1005".to_string(), "CRPQC".to_string()); // CRP QC count results
    codes.insert("1006".to_string(), "XbarRQC".to_string()); // Xbar-R QC count results
    
    codes
}

// ============================================================================
// HL7 DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HL7Message {
    pub message_type: String,
    pub message_control_id: String,
    pub processing_id: String,
    pub version_id: String,
    pub segments: Vec<HL7Segment>,
    pub raw_message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HL7Segment {
    pub segment_type: String,
    pub fields: Vec<String>,
    pub raw_segment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MSHSegment {
    pub field_separator: String,
    pub encoding_characters: String,
    pub sending_application: String,
    pub sending_facility: String,
    pub receiving_application: String,
    pub receiving_facility: String,
    pub date_time_of_message: String,
    pub security: String,
    pub message_type: String,
    pub message_control_id: String,
    pub processing_id: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIDSegment {
    pub set_id: String,
    pub patient_id: String,
    pub patient_identifier_list: String,
    pub alternate_patient_id: String,
    pub patient_name: String,
    pub mothers_maiden_name: String,
    pub date_time_of_birth: String,
    pub administrative_sex: String,
    pub patient_alias: String,
    pub race: String,
    pub patient_address: String,
    pub county_code: String,
    pub phone_number_home: String,
    pub phone_number_business: String,
    pub primary_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OBRSegment {
    pub set_id: String,
    pub placer_order_number: String,
    pub filler_order_number: String,
    pub universal_service_identifier: String,
    pub priority: String,
    pub requested_date_time: String,
    pub observation_date_time: String,
    pub observation_end_date_time: String,
    pub collection_volume: String,
    pub collector_identifier: String,
    pub specimen_action_code: String,
    pub danger_code: String,
    pub relevant_clinical_information: String,
    pub specimen_received_date_time: String,
    pub specimen_source: String,
    pub ordering_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OBXSegment {
    pub set_id: String,
    pub value_type: String,
    pub observation_identifier: String,
    pub observation_sub_id: String,
    pub observation_value: String,
    pub units: String,
    pub references_range: String,
    pub abnormal_flags: String,
    pub probability: String,
    pub nature_of_abnormal_test: String,
    pub observation_result_status: String,
    pub effective_date_of_reference_range: String,
    pub user_defined_access_checks: String,
    pub date_time_of_observation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MSASegment {
    pub acknowledgment_code: String,
    pub message_control_id: String,
    pub text_message: String,
    pub expected_sequence_number: String,
    pub delayed_acknowledgment_type: String,
    pub error_condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ORCSegment {
    pub order_control: String,
    pub placer_order_number: String,
    pub filler_order_number: String,
    pub placer_group_number: String,
    pub order_status: String,
    pub response_flag: String,
    pub quantity_timing: String,
    pub parent_order: String,
    pub date_time_of_transaction: String,
    pub entered_by: String,
    pub verified_by: String,
    pub ordering_provider: String,
}

// ============================================================================
// CONNECTION STATE FOR HL7/MLLP
// ============================================================================

#[derive(Debug, Clone)]
pub enum HL7ConnectionState {
    WaitingForStartBlock,    // Waiting for MLLP VT (0x0B)
    ReadingMessage,          // Reading HL7 message content
    WaitingForEndBlock,      // Waiting for MLLP FS (0x1C)
    WaitingForCarriageReturn, // Waiting for MLLP CR (0x0D)
    ProcessingMessage,       // Processing complete HL7 message
    SendingAck,             // Sending acknowledgment
    Complete,               // Message processing complete
}

// ============================================================================
// HL7 PARSING FUNCTIONS
// ============================================================================

/// Extracts HL7 message content from MLLP frame
pub fn extract_mllp_message(data: &[u8]) -> Result<Vec<u8>, String> {
    // Find the start block (VT)
    let start_pos = data.iter().position(|&b| b == MLLP_START_BLOCK)
        .ok_or("MLLP start block not found")?;

    // Find the end sequence (FS CR)
    let mut end_pos = None;
    for i in start_pos + 1..data.len() - 1 {
        if data[i] == MLLP_END_BLOCK && data[i + 1] == MLLP_CARRIAGE_RETURN {
            end_pos = Some(i);
            break;
        }
    }

    let end_pos = end_pos.ok_or("MLLP end sequence not found")?;
    
    // Extract message content between start and end blocks
    let message_content = data[start_pos + 1..end_pos].to_vec();
    
    Ok(message_content)
}

/// Creates MLLP frame around HL7 message
pub fn create_mllp_frame(hl7_message: &str) -> Vec<u8> {
    let mut frame = Vec::new();
    
    // Add start block
    frame.push(MLLP_START_BLOCK);
    
    // Add HL7 message
    frame.extend_from_slice(hl7_message.as_bytes());
    
    // Add end sequence
    frame.push(MLLP_END_BLOCK);
    frame.push(MLLP_CARRIAGE_RETURN);
    
    frame
}

/// Validates MLLP frame structure
pub fn validate_mllp_frame(data: &[u8]) -> bool {
    if data.len() < 3 {
        return false;
    }
    
    // Check for start block
    if data[0] != MLLP_START_BLOCK {
        return false;
    }
    
    // Check for end sequence
    let len = data.len();
    if len >= 2 && data[len - 2] == MLLP_END_BLOCK && data[len - 1] == MLLP_CARRIAGE_RETURN {
        return true;
    }
    
    false
}

/// Parses HL7 message from string
pub fn parse_hl7_message(message_content: &str) -> Result<HL7Message, String> {
    if message_content.is_empty() {
        return Err("Empty HL7 message".to_string());
    }
    
    // Split message into segments by carriage return
    let segment_lines: Vec<&str> = message_content.split('\r').collect();
    
    if segment_lines.is_empty() {
        return Err("No segments found in HL7 message".to_string());
    }
    
    let mut segments = Vec::new();
    let mut message_type = String::new();
    let mut message_control_id = String::new();
    let mut processing_id = String::new();
    let mut version_id = String::new();
    
    // Parse each segment
    for segment_line in segment_lines {
        if segment_line.trim().is_empty() {
            continue;
        }
        
        let segment = parse_hl7_segment(segment_line)?;
        
        // Extract metadata from MSH segment
        if segment.segment_type == "MSH" {
            let msh = parse_msh_segment(&segment)?;
            message_type = msh.message_type;
            message_control_id = msh.message_control_id;
            processing_id = msh.processing_id;
            version_id = msh.version_id;
        }
        
        segments.push(segment);
    }
    
    Ok(HL7Message {
        message_type,
        message_control_id,
        processing_id,
        version_id,
        segments,
        raw_message: message_content.to_string(),
        timestamp: Utc::now(),
    })
}

/// Parses individual HL7 segment
pub fn parse_hl7_segment(segment_line: &str) -> Result<HL7Segment, String> {
    if segment_line.len() < 3 {
        return Err("Segment too short".to_string());
    }
    
    let segment_type = &segment_line[0..3];
    
    // Split by field separator (|)
    let fields: Vec<String> = segment_line
        .split(HL7_FIELD_SEPARATOR)
        .map(|s| s.to_string())
        .collect();
    
    Ok(HL7Segment {
        segment_type: segment_type.to_string(),
        fields,
        raw_segment: segment_line.to_string(),
    })
}

/// Parses MSH (Message Header) segment
pub fn parse_msh_segment(segment: &HL7Segment) -> Result<MSHSegment, String> {
    if segment.segment_type != "MSH" {
        return Err("Not an MSH segment".to_string());
    }
    
    if segment.fields.len() < 12 {
        return Err("MSH segment has insufficient fields".to_string());
    }
    
    Ok(MSHSegment {
        field_separator: segment.fields.get(1).unwrap_or(&String::new()).clone(),
        encoding_characters: segment.fields.get(1).unwrap_or(&String::new()).clone(), // MSH.2 is actually field separator + encoding chars
        sending_application: segment.fields.get(2).unwrap_or(&String::new()).clone(), // MSH.3
        sending_facility: segment.fields.get(3).unwrap_or(&String::new()).clone(),     // MSH.4
        receiving_application: segment.fields.get(4).unwrap_or(&String::new()).clone(), // MSH.5
        receiving_facility: segment.fields.get(5).unwrap_or(&String::new()).clone(),    // MSH.6
        date_time_of_message: segment.fields.get(6).unwrap_or(&String::new()).clone(),  // MSH.7
        security: segment.fields.get(7).unwrap_or(&String::new()).clone(),              // MSH.8
        message_type: segment.fields.get(8).unwrap_or(&String::new()).clone(),          // MSH.9
        message_control_id: segment.fields.get(9).unwrap_or(&String::new()).clone(),    // MSH.10
        processing_id: segment.fields.get(10).unwrap_or(&String::new()).clone(),        // MSH.11
        version_id: segment.fields.get(11).unwrap_or(&String::new()).clone(),           // MSH.12
    })
}

/// Parses PID (Patient Identification) segment
pub fn parse_pid_segment(segment: &HL7Segment) -> Result<PIDSegment, String> {
    if segment.segment_type != "PID" {
        return Err("Not a PID segment".to_string());
    }
    
    Ok(PIDSegment {
        set_id: segment.fields.get(1).unwrap_or(&String::new()).clone(),
        patient_id: segment.fields.get(2).unwrap_or(&String::new()).clone(),
        patient_identifier_list: segment.fields.get(3).unwrap_or(&String::new()).clone(),
        alternate_patient_id: segment.fields.get(4).unwrap_or(&String::new()).clone(),
        patient_name: segment.fields.get(5).unwrap_or(&String::new()).clone(),
        mothers_maiden_name: segment.fields.get(6).unwrap_or(&String::new()).clone(),
        date_time_of_birth: segment.fields.get(7).unwrap_or(&String::new()).clone(),
        administrative_sex: segment.fields.get(8).unwrap_or(&String::new()).clone(),
        patient_alias: segment.fields.get(9).unwrap_or(&String::new()).clone(),
        race: segment.fields.get(10).unwrap_or(&String::new()).clone(),
        patient_address: segment.fields.get(11).unwrap_or(&String::new()).clone(),
        county_code: segment.fields.get(12).unwrap_or(&String::new()).clone(),
        phone_number_home: segment.fields.get(13).unwrap_or(&String::new()).clone(),
        phone_number_business: segment.fields.get(14).unwrap_or(&String::new()).clone(),
        primary_language: segment.fields.get(15).unwrap_or(&String::new()).clone(),
    })
}

/// Parses OBR (Observation Request) segment
pub fn parse_obr_segment(segment: &HL7Segment) -> Result<OBRSegment, String> {
    if segment.segment_type != "OBR" {
        return Err("Not an OBR segment".to_string());
    }
    
    Ok(OBRSegment {
        set_id: segment.fields.get(1).unwrap_or(&String::new()).clone(),
        placer_order_number: segment.fields.get(2).unwrap_or(&String::new()).clone(),
        filler_order_number: segment.fields.get(3).unwrap_or(&String::new()).clone(),
        universal_service_identifier: segment.fields.get(4).unwrap_or(&String::new()).clone(),
        priority: segment.fields.get(5).unwrap_or(&String::new()).clone(),
        requested_date_time: segment.fields.get(6).unwrap_or(&String::new()).clone(),
        observation_date_time: segment.fields.get(7).unwrap_or(&String::new()).clone(),
        observation_end_date_time: segment.fields.get(8).unwrap_or(&String::new()).clone(),
        collection_volume: segment.fields.get(9).unwrap_or(&String::new()).clone(),
        collector_identifier: segment.fields.get(10).unwrap_or(&String::new()).clone(),
        specimen_action_code: segment.fields.get(11).unwrap_or(&String::new()).clone(),
        danger_code: segment.fields.get(12).unwrap_or(&String::new()).clone(),
        relevant_clinical_information: segment.fields.get(13).unwrap_or(&String::new()).clone(),
        specimen_received_date_time: segment.fields.get(14).unwrap_or(&String::new()).clone(),
        specimen_source: segment.fields.get(15).unwrap_or(&String::new()).clone(),
        ordering_provider: segment.fields.get(16).unwrap_or(&String::new()).clone(),
    })
}

/// Parses OBX (Observation Result) segment
pub fn parse_obx_segment(segment: &HL7Segment) -> Result<OBXSegment, String> {
    if segment.segment_type != "OBX" {
        return Err("Not an OBX segment".to_string());
    }
    
    Ok(OBXSegment {
        set_id: segment.fields.get(1).unwrap_or(&String::new()).clone(),
        value_type: segment.fields.get(2).unwrap_or(&String::new()).clone(),
        observation_identifier: segment.fields.get(3).unwrap_or(&String::new()).clone(),
        observation_sub_id: segment.fields.get(4).unwrap_or(&String::new()).clone(),
        observation_value: segment.fields.get(5).unwrap_or(&String::new()).clone(),
        units: segment.fields.get(6).unwrap_or(&String::new()).clone(),
        references_range: segment.fields.get(7).unwrap_or(&String::new()).clone(),
        abnormal_flags: segment.fields.get(8).unwrap_or(&String::new()).clone(),
        probability: segment.fields.get(9).unwrap_or(&String::new()).clone(),
        nature_of_abnormal_test: segment.fields.get(10).unwrap_or(&String::new()).clone(),
        observation_result_status: segment.fields.get(11).unwrap_or(&String::new()).clone(),
        effective_date_of_reference_range: segment.fields.get(12).unwrap_or(&String::new()).clone(),
        user_defined_access_checks: segment.fields.get(13).unwrap_or(&String::new()).clone(),
        date_time_of_observation: segment.fields.get(14).unwrap_or(&String::new()).clone(),
    })
}

/// Parses MSA (Message Acknowledgment) segment
pub fn parse_msa_segment(segment: &HL7Segment) -> Result<MSASegment, String> {
    if segment.segment_type != "MSA" {
        return Err("Not an MSA segment".to_string());
    }
    
    Ok(MSASegment {
        acknowledgment_code: segment.fields.get(1).unwrap_or(&String::new()).clone(),
        message_control_id: segment.fields.get(2).unwrap_or(&String::new()).clone(),
        text_message: segment.fields.get(3).unwrap_or(&String::new()).clone(),
        expected_sequence_number: segment.fields.get(4).unwrap_or(&String::new()).clone(),
        delayed_acknowledgment_type: segment.fields.get(5).unwrap_or(&String::new()).clone(),
        error_condition: segment.fields.get(6).unwrap_or(&String::new()).clone(),
    })
}

/// Parses ORC (Common Order) segment
pub fn parse_orc_segment(segment: &HL7Segment) -> Result<ORCSegment, String> {
    if segment.segment_type != "ORC" {
        return Err("Not an ORC segment".to_string());
    }
    
    Ok(ORCSegment {
        order_control: segment.fields.get(1).unwrap_or(&String::new()).clone(),
        placer_order_number: segment.fields.get(2).unwrap_or(&String::new()).clone(),
        filler_order_number: segment.fields.get(3).unwrap_or(&String::new()).clone(),
        placer_group_number: segment.fields.get(4).unwrap_or(&String::new()).clone(),
        order_status: segment.fields.get(5).unwrap_or(&String::new()).clone(),
        response_flag: segment.fields.get(6).unwrap_or(&String::new()).clone(),
        quantity_timing: segment.fields.get(7).unwrap_or(&String::new()).clone(),
        parent_order: segment.fields.get(8).unwrap_or(&String::new()).clone(),
        date_time_of_transaction: segment.fields.get(9).unwrap_or(&String::new()).clone(),
        entered_by: segment.fields.get(10).unwrap_or(&String::new()).clone(),
        verified_by: segment.fields.get(11).unwrap_or(&String::new()).clone(),
        ordering_provider: segment.fields.get(12).unwrap_or(&String::new()).clone(),
    })
}

/// Creates HL7 ACK (Acknowledgment) message for CQ 5 Plus (HL7 v2.3.1)
pub fn create_hl7_acknowledgment(
    original_message: &HL7Message,
    ack_code: &str,
    text_message: Option<&str>,
) -> String {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let control_id = format!("ACK{}", timestamp);
    
    // MSH segment for ACK (HL7 v2.3.1)
    let msh = format!(
        "MSH|^~\\&|LIS|HOSPITAL|{}|{}|{}||ACK^{}^ACK|{}|P|2.3.1||||||UTF-8",
        original_message.segments.first()
            .and_then(|s| s.fields.get(3))
            .unwrap_or(&"SENDER".to_string()),
        original_message.segments.first()
            .and_then(|s| s.fields.get(4))
            .unwrap_or(&"FACILITY".to_string()),
        timestamp,
        original_message.message_type.split('^').next().unwrap_or("R01"),
        control_id
    );
    
    // MSA segment for acknowledgment
    let msa = format!(
        "MSA|{}|{}|{}",
        ack_code,
        original_message.message_control_id,
        text_message.unwrap_or("")
    );
    
    format!("{}\r{}\r", msh, msa)
}

/// Determines processing ID based on message type (CQ 5 Plus logic)
pub fn get_processing_id_for_message_type(message_type: &str, obr_service_code: Option<&str>) -> String {
    // For QC messages, use "Q"
    if let Some(service_code) = obr_service_code {
        if service_code.contains("QC") || service_code.contains("1002") || 
           service_code.contains("1003") || service_code.contains("1004") || 
           service_code.contains("1005") || service_code.contains("1006") {
            return "Q".to_string();
        }
    }
    
    // For regular samples, use "P"
    match message_type {
        t if t.starts_with("ORU") => "P".to_string(),
        t if t.starts_with("OUL") => "Q".to_string(), // OUL is typically QC
        t if t.starts_with("ORM") => "P".to_string(), // Worklist request
        t if t.starts_with("ORR") => "P".to_string(), // Worklist response
        _ => "P".to_string(), // Default to sample processing
    }
}

/// Validates message type support (CQ 5 Plus supported types)
pub fn is_supported_message_type(message_type: &str) -> bool {
    match message_type {
        "ORU^R01" => true,  // Observation result
        "OUL^R21" => true,  // Unsolicited observation (QC)
        "ORM^O01" => true,  // Order message (worklist request)
        "ORR^O02" => true,  // Order response (worklist response)
        "ACK" => true,      // Acknowledgment
        _ => false,
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Extracts hematology parameter name from observation identifier (CQ 5 Plus codes)
pub fn extract_parameter_name(observation_identifier: &str) -> String {
    // Parse observation identifier field (typically contains code^text^coding_system)
    let parts: Vec<&str> = observation_identifier.split(HL7_COMPONENT_SEPARATOR).collect();
    
    if parts.len() >= 2 {
        parts[1].to_string() // Return the text component
    } else if !parts.is_empty() {
        // Try to map code to parameter name using CQ 5 Plus codes
        let code = parts[0];
        let parameter_codes = get_cq5_parameter_codes();
        parameter_codes.get(code).cloned().unwrap_or_else(|| parts[0].to_string())
    } else {
        "Unknown".to_string()
    }
}

/// Extracts parameter code from observation identifier
pub fn extract_parameter_code(observation_identifier: &str) -> String {
    let parts: Vec<&str> = observation_identifier.split(HL7_COMPONENT_SEPARATOR).collect();
    if !parts.is_empty() {
        parts[0].to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Checks if parameter is a CRP-related test (new in CQ 5 Plus)
pub fn is_crp_parameter(parameter_code: &str) -> bool {
    matches!(parameter_code, "2031" | "2032")
}

/// Checks if parameter is histogram/scattergram data
pub fn is_histogram_parameter(parameter_code: &str) -> bool {
    matches!(parameter_code, "2101" | "2102" | "2033" | "2034")
}

/// Extracts flags from abnormal flags field
pub fn extract_abnormal_flags(abnormal_flags: &str) -> Vec<String> {
    if abnormal_flags.is_empty() {
        return Vec::new();
    }
    
    abnormal_flags
        .split(HL7_REPETITION_SEPARATOR)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mllp_frame_creation() {
        let message = "MSH|^~\\&|BF-6900|20180613001|LIS|RECEIVER|20240101120000||ORU^R01|123456|P|2.3.1||||||UTF-8\rPID|1||P123456|||DOE^JOHN||19800101|M\r";
        let frame = create_mllp_frame(message);
        
        assert_eq!(frame[0], MLLP_START_BLOCK);
        assert_eq!(frame[frame.len() - 2], MLLP_END_BLOCK);
        assert_eq!(frame[frame.len() - 1], MLLP_CARRIAGE_RETURN);
    }

    #[test]
    fn test_mllp_frame_validation() {
        let valid_frame = vec![MLLP_START_BLOCK, b'T', b'E', b'S', b'T', MLLP_END_BLOCK, MLLP_CARRIAGE_RETURN];
        assert!(validate_mllp_frame(&valid_frame));
        
        let invalid_frame = vec![b'T', b'E', b'S', b'T'];
        assert!(!validate_mllp_frame(&invalid_frame));
    }

    #[test]
    fn test_mllp_message_extraction() {
        let frame = vec![MLLP_START_BLOCK, b'T', b'E', b'S', b'T', MLLP_END_BLOCK, MLLP_CARRIAGE_RETURN];
        let extracted = extract_mllp_message(&frame).unwrap();
        assert_eq!(extracted, vec![b'T', b'E', b'S', b'T']);
    }

    #[test]
    fn test_hl7_segment_parsing() {
        let segment_line = "MSH|^~\\&|BF-6900|20180613001|LIS|RECEIVER|20240101120000||ORU^R01|123456|P|2.3.1||||||UTF-8";
        let segment = parse_hl7_segment(segment_line).unwrap();
        
        assert_eq!(segment.segment_type, "MSH");
        assert_eq!(segment.fields.len(), 12);
        assert_eq!(segment.fields[0], "MSH");
    }

    #[test]
    fn test_msh_segment_parsing() {
        let segment_line = "MSH|^~\\&|BF-6900|20180613001|LIS|RECEIVER|20240101120000||ORU^R01|123456|P|2.3.1||||||UTF-8";
        let segment = parse_hl7_segment(segment_line).unwrap();
        let msh = parse_msh_segment(&segment).unwrap();
        
        assert_eq!(msh.sending_application, "BF-6900");
        assert_eq!(msh.sending_facility, "20180613001");
        assert_eq!(msh.message_type, "ORU^R01");
        assert_eq!(msh.message_control_id, "123456");
        assert_eq!(msh.version_id, "2.3.1");
    }

    #[test]
    fn test_obx_segment_parsing() {
        let segment_line = "OBX|1|NM|2006^V_WBC^LOCAL|1|8.5|10^9/L|4.0-10.0|N|||F|||20240101120000";
        let segment = parse_hl7_segment(segment_line).unwrap();
        let obx = parse_obx_segment(&segment).unwrap();
        
        assert_eq!(obx.observation_identifier, "2006^V_WBC^LOCAL");
        assert_eq!(obx.observation_value, "8.5");
        assert_eq!(obx.units, "10^9/L");
        assert_eq!(obx.references_range, "4.0-10.0");
    }

    #[test]
    fn test_parameter_name_extraction() {
        let observation_id = "2006^V_WBC^LOCAL";
        let parameter = extract_parameter_name(observation_id);
        assert_eq!(parameter, "V_WBC");
        
        let simple_id = "2006";
        let simple_parameter = extract_parameter_name(simple_id);
        assert_eq!(simple_parameter, "V_WBC");
    }

    #[test]
    fn test_abnormal_flags_extraction() {
        let flags = "H~A";
        let extracted = extract_abnormal_flags(flags);
        assert_eq!(extracted, vec!["H", "A"]);
        
        let single_flag = "L";
        let single_extracted = extract_abnormal_flags(single_flag);
        assert_eq!(single_extracted, vec!["L"]);
        
        let empty_flags = "";
        let empty_extracted = extract_abnormal_flags(empty_flags);
        assert!(empty_extracted.is_empty());
    }

    #[test]
    fn test_hl7_ack_creation() {
        let message = HL7Message {
            message_type: "ORU^R01".to_string(),
            message_control_id: "123456".to_string(),
            processing_id: "P".to_string(),
            version_id: "2.4".to_string(),
            segments: vec![
                HL7Segment {
                    segment_type: "MSH".to_string(),
                    fields: vec![
                        "MSH".to_string(),
                        "|".to_string(),
                        "^~\\&".to_string(),
                        "LAB".to_string(),
                        "HOSPITAL".to_string(),
                    ],
                    raw_segment: "".to_string(),
                }
            ],
            raw_message: "".to_string(),
            timestamp: Utc::now(),
        };
        
        let ack = create_hl7_acknowledgment(&message, "AA", Some("Message accepted"));
        assert!(ack.contains("MSH|"));
        assert!(ack.contains("MSA|AA|123456|Message accepted"));
        assert!(ack.contains("2.3.1")); // Check HL7 version
        assert!(ack.contains("UTF-8")); // Check character set
    }

    #[test]
    fn test_cq5_parameter_codes() {
        let codes = get_cq5_parameter_codes();
        
        // Test some key parameters
        assert_eq!(codes.get("2006"), Some(&"V_WBC".to_string()));
        assert_eq!(codes.get("2031"), Some(&"V_CRP".to_string())); // New CRP parameter
        assert_eq!(codes.get("2032"), Some(&"V_HS_CRP".to_string())); // New HS-CRP parameter
        assert_eq!(codes.get("2030"), Some(&"V_P_LCC".to_string())); // New platelet parameter
    }

    #[test]
    fn test_processing_id_logic() {
        // Sample messages should use "P"
        assert_eq!(get_processing_id_for_message_type("ORU^R01", Some("1001^CountResults")), "P");
        
        // QC messages should use "Q"
        assert_eq!(get_processing_id_for_message_type("OUL^R21", Some("1002^LJQC")), "Q");
        assert_eq!(get_processing_id_for_message_type("ORU^R01", Some("1003^XbarQC")), "Q");
        
        // Worklist messages should use "P"
        assert_eq!(get_processing_id_for_message_type("ORM^O01", None), "P");
        assert_eq!(get_processing_id_for_message_type("ORR^O02", None), "P");
    }

    #[test]
    fn test_supported_message_types() {
        assert!(is_supported_message_type("ORU^R01"));
        assert!(is_supported_message_type("OUL^R21"));
        assert!(is_supported_message_type("ORM^O01")); // Worklist request
        assert!(is_supported_message_type("ORR^O02")); // Worklist response
        assert!(is_supported_message_type("ACK"));
        
        assert!(!is_supported_message_type("INVALID^TYPE"));
    }

    #[test]
    fn test_crp_parameter_detection() {
        assert!(is_crp_parameter("2031")); // V_CRP
        assert!(is_crp_parameter("2032")); // V_HS_CRP
        assert!(!is_crp_parameter("2006")); // V_WBC
    }

    #[test]
    fn test_histogram_parameter_detection() {
        assert!(is_histogram_parameter("2101")); // RBC histogram
        assert!(is_histogram_parameter("2034")); // DIFF scattergram
        assert!(!is_histogram_parameter("2006")); // V_WBC
    }

    #[test]
    fn test_msa_segment_parsing() {
        let segment_line = "MSA|AA|123456|Message accepted|||0";
        let segment = parse_hl7_segment(segment_line).unwrap();
        let msa = parse_msa_segment(&segment).unwrap();
        
        assert_eq!(msa.acknowledgment_code, "AA");
        assert_eq!(msa.message_control_id, "123456");
        assert_eq!(msa.text_message, "Message accepted");
        assert_eq!(msa.error_condition, "0");
    }

    #[test]
    fn test_orc_segment_parsing() {
        let segment_line = "ORC|RF||SampleID||IP";
        let segment = parse_hl7_segment(segment_line).unwrap();
        let orc = parse_orc_segment(&segment).unwrap();
        
        assert_eq!(orc.order_control, "RF");
        assert_eq!(orc.filler_order_number, "SampleID");
        assert_eq!(orc.order_status, "IP");
    }
}