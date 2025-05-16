use crate::model::patient::Patient;
use crate::model::result::TestResult;
use crate::protocol::astm::constants::{ACK, ENQ, EOT, NAK};
use crate::protocol::astm::{AstmProtocol, Frame, Record, RecordType};
use crate::protocol::error::{ProtocolError, Result as ProtocolResult};
use crate::service::patient_service::PatientService;
use crate::service::result_service::ResultService;
use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{debug, error, info, trace, warn};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use uuid::Uuid;

/// Service for handling Meril machine communication
pub struct MerilMachineService {
    patient_service: Arc<PatientService>,
    result_service: Arc<ResultService>,
    port: u16,
    is_running: Arc<AtomicBool>,
}

impl MerilMachineService {
    /// Create a new Meril machine service
    pub fn new(
        patient_service: Arc<PatientService>,
        result_service: Arc<ResultService>,
        port: u16,
    ) -> Self {
        info!("Creating new MerilMachineService on port {}", port);
        Self {
            patient_service,
            result_service,
            port,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get the port the service is running on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Check if the service is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    /// Start the TCP server for ASTM communication
    pub async fn start_server(&self) -> Result<()> {
        if self.is_running.load(Ordering::SeqCst) {
            info!("Attempted to start server that is already running");
            return Err(anyhow!("Server is already running"));
        }

        let patient_service = Arc::clone(&self.patient_service);
        let result_service = Arc::clone(&self.result_service);
        let port = self.port;
        let is_running = Arc::clone(&self.is_running);

        // Set the running state to true
        is_running.store(true, Ordering::SeqCst);

        info!("Starting ASTM TCP server on port {}", port);

        // Spawn a background task for the TCP server
        task::spawn(async move {
            // Move to a blocking task for TcpListener to work properly
            let server_handle = task::spawn_blocking(move || {
                match Self::run_tcp_server(patient_service, result_service, port, is_running) {
                    Ok(_) => info!("TCP server shutdown normally"),
                    Err(e) => error!("TCP server error: {}", e),
                }
            });

            if let Err(e) = server_handle.await {
                error!("Server task failed: {}", e);
            }
        });

        info!("ASTM TCP server task spawned successfully");
        Ok(())
    }

    /// Stop the TCP server
    pub fn stop_server(&self) -> Result<()> {
        if !self.is_running.load(Ordering::SeqCst) {
            info!("Attempted to stop server that is not running");
            return Err(anyhow!("Server is not running"));
        }

        info!("Stopping ASTM TCP server on port {}", self.port);
        // Set the running state to false, which will cause run_tcp_server to exit
        self.is_running.store(false, Ordering::SeqCst);
        info!("ASTM TCP server shutdown initiated");
        Ok(())
    }

    /// Run the TCP server in a blocking manner
    fn run_tcp_server(
        patient_service: Arc<PatientService>,
        result_service: Arc<ResultService>,
        port: u16,
        is_running: Arc<AtomicBool>,
    ) -> Result<()> {
        // Create a TCP listener
        let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
            Ok(listener) => {
                info!("TCP listener bound to 0.0.0.0:{}", port);
                listener
            }
            Err(e) => {
                error!("Failed to bind TCP listener to port {}: {}", port, e);
                return Err(anyhow!(e));
            }
        };

        // Set non-blocking mode so we can check is_running periodically
        // if let Err(e) = listener.set_nonblocking(true) {
        //     error!("Failed to set non-blocking mode: {}", e);
        //     return Err(anyhow!(e));
        // }

        info!(
            "Meril TCP server started on port {} and ready for connections",
            port
        );

        let mut connection_count = 0;

        while is_running.load(Ordering::SeqCst) {
            // Accept connections with a timeout
            match listener.accept() {
                Ok((stream, addr)) => {
                    connection_count += 1;
                    info!(
                        "New connection #{} accepted from: {}",
                        connection_count, addr
                    );

                    // Set timeouts
                    if let Err(e) = stream.set_read_timeout(Some(Duration::from_millis(5000))) {
                        error!("Failed to set read timeout: {}", e);
                        continue;
                    }

                    if let Err(e) = stream.set_write_timeout(Some(Duration::from_millis(5000))) {
                        error!("Failed to set write timeout: {}", e);
                        continue;
                    }

                    debug!("Connection timeouts set for client {}", addr);

                    // Clone services for the handler thread
                    let handler_patient_service = Arc::clone(&patient_service);
                    let handler_result_service = Arc::clone(&result_service);

                    // Spawn a thread to handle this connection
                    std::thread::spawn(move || {
                        info!("Starting connection handler for client {}", addr);
                        if let Err(e) = Self::handle_connection(
                            stream,
                            handler_patient_service,
                            handler_result_service,
                        ) {
                            error!("Connection handler error for {}: {}", addr, e);
                        } else {
                            info!("Connection handler completed successfully for {}", addr);
                        }
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No connection available, sleep a bit to avoid CPU spin
                    trace!("No connection available, sleeping briefly");
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                    // Sleep to avoid rapid error logging
                    std::thread::sleep(Duration::from_millis(1000));
                }
            }
        }

        info!("TCP server is shutting down");
        Ok(())
    }

    /// Handle a single TCP connection
    fn handle_connection(
        mut stream: TcpStream,
        patient_service: Arc<PatientService>,
        result_service: Arc<ResultService>,
    ) -> Result<()> {
        let addr = match stream.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".to_string(),
        };

        let mut buffer = [0; 1024];

        debug!("[{}] Waiting for ENQ", addr);

        // Wait for ENQ
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    info!("[{}] Connection closed while waiting for ENQ", addr);
                    return Err(anyhow!("Connection closed"));
                }
                Ok(bytes_read) => {
                    if bytes_read > 0 && buffer[0] == ENQ {
                        info!("[{}] Received ENQ", addr);
                        // Send ACK
                        stream.write_all(&[ACK])?;
                        info!("[{}] Sent ACK", addr);
                        break;
                    } else if bytes_read > 0 {
                        warn!(
                            "[{}] Received unexpected data while waiting for ENQ: {:?}",
                            addr,
                            &buffer[0..bytes_read]
                        );
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        trace!("[{}] Timeout while waiting for ENQ, continuing", addr);
                        continue; // Keep waiting for ENQ
                    }
                    error!("[{}] Read error while waiting for ENQ: {}", addr, e);
                    return Err(anyhow!("Read error: {}", e));
                }
            }
        }

        // Process frames
        let mut frames = Vec::new();
        let mut eot_received = false;

        info!("[{}] Starting to process data frames", addr);

        while !eot_received {
            // Read next message
            match stream.read(&mut buffer) {
                Ok(0) => {
                    warn!(
                        "[{}] Connection closed unexpectedly during frame processing",
                        addr
                    );
                    return Err(anyhow!("Connection closed unexpectedly"));
                }
                Ok(bytes_read) => {
                    // Check for EOT
                    if bytes_read == 1 && buffer[0] == EOT {
                        info!("[{}] Received EOT", addr);
                        eot_received = true;
                        break;
                    }

                    // Try to parse as a frame
                    if bytes_read > 0 && buffer[0] == 0x02 {
                        // STX
                        let frame_data = &buffer[0..bytes_read];
                        debug!(
                            "[{}] Attempting to parse frame data of length {}",
                            addr, bytes_read
                        );

                        match Frame::parse(frame_data) {
                            Ok(frame) => {
                                info!("[{}] Received frame with sequence {}", addr, frame.sequence);

                                // Send ACK
                                if let Err(e) = stream.write_all(&[ACK]) {
                                    error!("[{}] Failed to send ACK: {}", addr, e);
                                    return Err(anyhow!("Write error: {}", e));
                                }
                                info!("[{}] Sent ACK for frame {}", addr, frame.sequence);

                                // Store the frame
                                frames.push(frame);
                                debug!(
                                    "[{}] Stored frame, total frames now: {}",
                                    addr,
                                    frames.len()
                                );
                            }
                            Err(e) => {
                                warn!("[{}] Invalid frame: {}", addr, e);
                                // Send NAK
                                if let Err(e) = stream.write_all(&[NAK]) {
                                    error!("[{}] Failed to send NAK: {}", addr, e);
                                    return Err(anyhow!("Write error: {}", e));
                                }
                                info!("[{}] Sent NAK", addr);
                            }
                        }
                    } else if bytes_read > 0 {
                        warn!(
                            "[{}] Received unexpected data: first byte 0x{:02X}",
                            addr, buffer[0]
                        );
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        trace!(
                            "[{}] Timeout while waiting for next frame, continuing",
                            addr
                        );
                        continue; // Keep waiting
                    }
                    error!("[{}] Read error during frame processing: {}", addr, e);
                    return Err(anyhow!("Read error: {}", e));
                }
            }
        }

        // Process received frames
        if !frames.is_empty() && eot_received {
            info!("[{}] Processing {} received frames", addr, frames.len());
            let result = Self::process_frames(frames, patient_service, result_service);
            if let Err(e) = result {
                error!("[{}] Error processing frames: {}", addr, e);
            } else {
                info!("[{}] Successfully processed all frames", addr);
            }
        } else if frames.is_empty() {
            warn!("[{}] No frames received before EOT", addr);
        } else if !eot_received {
            warn!("[{}] Did not receive EOT after frames", addr);
        }

        info!("[{}] Connection handler completed", addr);
        Ok(())
    }

    /// Process the received frames
    fn process_frames(
        frames: Vec<Frame>,
        patient_service: Arc<PatientService>,
        result_service: Arc<ResultService>,
    ) -> Result<()> {
        info!("Starting to process {} frames", frames.len());

        // Variables to store message components
        let mut current_patient: Option<Patient> = None;
        let mut results = Vec::new();

        // Process each frame
        for (frame_idx, frame) in frames.iter().enumerate() {
            debug!("Processing frame {} of {}", frame_idx + 1, frames.len());

            // Split the frame into records
            let records = match Self::split_frame_to_records(frame) {
                Ok(recs) => {
                    debug!("Frame {} split into {} records", frame_idx + 1, recs.len());
                    recs
                }
                Err(e) => {
                    error!("Failed to split frame {}: {}", frame_idx + 1, e);
                    return Err(anyhow!("Failed to split frame: {}", e));
                }
            };

            // Process each record
            for (rec_idx, record) in records.iter().enumerate() {
                debug!(
                    "Processing record {} of {} from frame {}",
                    rec_idx + 1,
                    records.len(),
                    frame_idx + 1
                );

                match record.record_type {
                    RecordType::Header => {
                        debug!("Processing header record from frame {}", frame_idx + 1);
                        // Header records are informational only
                        trace!("Header record content: {:?}", record.fields);
                    }
                    RecordType::Patient => {
                        info!("Processing patient record from frame {}", frame_idx + 1);
                        match Self::parse_patient_record(record) {
                            Ok(patient) => {
                                info!(
                                    "Successfully parsed patient record for patient ID: {}",
                                    patient.id
                                );
                                current_patient = Some(patient);
                            }
                            Err(e) => {
                                error!("Failed to parse patient record: {}", e);
                                return Err(anyhow!("Failed to parse patient record: {}", e));
                            }
                        }
                    }
                    RecordType::Result => {
                        info!("Processing result record from frame {}", frame_idx + 1);
                        match Self::parse_result_record(record) {
                            Ok(result) => {
                                info!(
                                    "Successfully parsed result record for test ID: {}",
                                    result.test_id
                                );
                                results.push(result);
                            }
                            Err(e) => {
                                error!("Failed to parse result record: {}", e);
                                return Err(anyhow!("Failed to parse result record: {}", e));
                            }
                        }
                    }
                    RecordType::Terminator => {
                        debug!("Processing terminator record from frame {}", frame_idx + 1);
                        // Terminator records are informational only
                    }
                    _ => {
                        // Ignore other record types
                        debug!(
                            "Ignoring record type: {:?} from frame {}",
                            record.record_type,
                            frame_idx + 1
                        );
                    }
                }
            }
        }

        // Save the patient if one was found
        if let Some(patient) = current_patient {
            info!(
                "Saving patient: {} with {} associated results",
                patient.id,
                results.len()
            );

            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    error!("Failed to create Tokio runtime: {}", e);
                    return Err(anyhow!("Failed to create Tokio runtime: {}", e));
                }
            };

            let patient_id = match rt.block_on(patient_service.save_patient(&patient)) {
                Ok(id) => {
                    info!("Patient saved with ID: {}", id);
                    id
                }
                Err(e) => {
                    error!("Failed to save patient: {}", e);
                    return Err(anyhow!("Failed to save patient: {}", e));
                }
            };

            // Save results with the patient ID
            for (idx, result) in results.iter().enumerate() {
                info!(
                    "Saving result {} of {}: test ID = {}",
                    idx + 1,
                    results.len(),
                    result.test_id
                );

                match rt.block_on(result_service.save_result(result)) {
                    Ok(id) => info!("Result saved with ID: {}", id),
                    Err(e) => {
                        error!("Failed to save result: {}", e);
                        // Continue trying to save other results
                        warn!("Continuing to process other results despite error");
                    }
                }
            }

            info!("Successfully processed all data");
        } else {
            warn!("No patient record found in frames, skipping result processing");
        }

        Ok(())
    }
}

impl AstmProtocol for MerilMachineService {
    fn parse_patient_record(record: &Record) -> ProtocolResult<Patient> {
        // Extract patient fields
        let id = record
            .get_field(3)
            .ok_or_else(|| ProtocolError::InvalidRecordFormat("Missing patient ID".to_string()))?
            .clone();

        // Parse patient name from field 5
        let name_str = record.get_field(5).cloned().unwrap_or_default();
        let name_components = Self::parse_components(&name_str);

        let last_name = if !name_components.is_empty() && !name_components[0].is_empty() {
            Some(name_components[0].clone())
        } else {
            None
        };

        let first_name = if name_components.len() > 1 && !name_components[1].is_empty() {
            Some(name_components[1].clone())
        } else {
            None
        };

        let middle_name = if name_components.len() > 2 && !name_components[2].is_empty() {
            Some(name_components[2].clone())
        } else {
            None
        };

        let title = if name_components.len() > 4 && !name_components[4].is_empty() {
            Some(name_components[4].clone())
        } else {
            None
        };

        // Parse birth date from field 7
        let birth_date = if let Some(date_str) = record.get_field(7) {
            Self::parse_datetime(date_str)
        } else {
            None
        };

        // Parse sex from field 8
        let sex_str = record
            .get_field(8)
            .cloned()
            .unwrap_or_else(|| "U".to_string());
        let sex = crate::model::patient::Sex::from(sex_str.as_str());

        // Parse address from field 11
        let address_str = record.get_field(11).cloned().unwrap_or_default();
        let address_components = Self::parse_components(&address_str);

        let address = if !address_components.is_empty() {
            Some(crate::model::patient::PatientAddress {
                street: if !address_components[0].is_empty() {
                    Some(address_components[0].clone())
                } else {
                    None
                },
                city: if address_components.len() > 3 && !address_components[3].is_empty() {
                    Some(address_components[3].clone())
                } else {
                    None
                },
                state: if address_components.len() > 4 && !address_components[4].is_empty() {
                    Some(address_components[4].clone())
                } else {
                    None
                },
                zip: if address_components.len() > 5 && !address_components[5].is_empty() {
                    Some(address_components[5].clone())
                } else {
                    None
                },
                country_code: if address_components.len() > 6 && !address_components[6].is_empty() {
                    Some(address_components[6].clone())
                } else {
                    None
                },
            })
        } else {
            None
        };

        // Parse phone number from field 13
        let mut telephone = Vec::new();
        if let Some(phone) = record.get_field(13) {
            if !phone.is_empty() {
                telephone.push(phone.clone());
            }
        }

        // Parse physician information from field 9
        let physicians_opt = if let Some(attending_physician) = record.get_field(9) {
            if !attending_physician.is_empty() {
                Some(crate::model::patient::PatientPhysicians {
                    attending: Some(attending_physician.clone()),
                    ordering: None,
                    referring: None,
                })
            } else {
                None
            }
        } else {
            None
        };

        let now = Utc::now();

        // Create the Patient object
        let patient = Patient {
            id,
            name: crate::model::patient::PatientName {
                last_name,
                first_name,
                middle_name,
                title,
            },
            birth_date,
            sex,
            address,
            telephone,
            physicians: physicians_opt,
            physical_attributes: None, // Not supported in basic ASTM
            created_at: now,
            updated_at: now,
        };

        Ok(patient)
    }

    fn parse_test_order_record(_record: &Record) -> ProtocolResult<crate::model::TestOrder> {
        // Implement test order parsing specific to Meril
        // TODO: Implement once TestOrder structure is available
        Err(ProtocolError::ProtocolError(
            "TestOrder parsing not implemented".to_string(),
        ))
    }

    fn parse_result_record(record: &Record) -> ProtocolResult<TestResult> {
        // Extract result fields
        let test_id = record
            .get_field(2)
            .ok_or_else(|| ProtocolError::InvalidRecordFormat("Missing test ID".to_string()))?
            .clone();

        let value = record
            .get_field(3)
            .ok_or_else(|| ProtocolError::InvalidRecordFormat("Missing result value".to_string()))?
            .clone();

        let units = record.get_field(4).cloned();

        // Parse reference range from field 5
        let reference_range = record.get_field(5).map(|range_str| {
            // Create a ReferenceRange struct from the string
            let parts: Vec<&str> = range_str.split('-').collect();
            crate::model::result::ReferenceRange {
                lower_limit: if parts.len() > 0 {
                    parts[0].parse::<f64>().ok()
                } else {
                    None
                },
                upper_limit: if parts.len() > 1 {
                    parts[1].parse::<f64>().ok()
                } else {
                    None
                },
            }
        });

        // Parse flags from field 6
        let flags = record
            .get_field(6)
            .map(|flag_str| crate::model::result::ResultFlags {
                abnormal_flag: Some(flag_str.clone()),
                nature_of_abnormality: None,
            });

        let sample_id = record
            .get_field(12)
            .cloned()
            .unwrap_or_else(|| format!("S-{}", Uuid::new_v4()));

        let analyzer_id = record.get_field(17).cloned();

        let now = Utc::now();

        // Create the TestResult object
        let result = TestResult {
            id: format!("R-{}", Uuid::new_v4()),
            test_id,
            sample_id,
            value,
            units,
            reference_range,
            flags,
            status: crate::model::result::ResultStatus::Final,
            completed_date_time: Some(now),
            metadata: crate::model::result::TestResultMetadata {
                sequence_number: 1,
                instrument: analyzer_id.clone(),
            },
            analyzer_id,
            created_at: now,
            updated_at: now,
        };

        Ok(result)
    }

    fn encode_patient_record(patient: &Patient) -> ProtocolResult<Record> {
        let mut record = Record::new(RecordType::Patient);

        // Sequence number
        record.set_field(1, "1".to_string());

        // Patient ID
        record.set_field(3, patient.id.clone());

        // Patient name
        let mut name_components = Vec::new();
        name_components.push(patient.name.last_name.clone().unwrap_or_default());
        name_components.push(patient.name.first_name.clone().unwrap_or_default());
        name_components.push(patient.name.middle_name.clone().unwrap_or_default());
        name_components.push("".to_string()); // Suffix (not used)
        name_components.push(patient.name.title.clone().unwrap_or_default());
        record.set_field(5, Self::join_components(&name_components));

        // Birth date
        if let Some(birth_date) = &patient.birth_date {
            record.set_field(7, Self::format_datetime(birth_date));
        }

        // Sex
        record.set_field(8, String::from(patient.sex.clone()));

        // Address
        if let Some(address) = &patient.address {
            let mut address_components = Vec::new();
            address_components.push(address.street.clone().unwrap_or_default());
            address_components.push("".to_string()); // Not used
            address_components.push("".to_string()); // Not used
            address_components.push(address.city.clone().unwrap_or_default());
            address_components.push(address.state.clone().unwrap_or_default());
            address_components.push(address.zip.clone().unwrap_or_default());
            address_components.push(address.country_code.clone().unwrap_or_default());
            record.set_field(11, Self::join_components(&address_components));
        }

        // Phone number
        if !patient.telephone.is_empty() {
            record.set_field(13, patient.telephone[0].clone());
        }

        // Physician
        if let Some(physicians) = &patient.physicians {
            if let Some(attending) = &physicians.attending {
                record.set_field(9, attending.clone());
            }
        }

        Ok(record)
    }

    fn encode_test_order_record(_order: &crate::model::TestOrder) -> ProtocolResult<Record> {
        // Implement test order encoding specific to Meril
        // TODO: Implement once TestOrder structure is available
        Err(ProtocolError::ProtocolError(
            "TestOrder encoding not implemented".to_string(),
        ))
    }

    fn encode_result_record(result: &TestResult) -> ProtocolResult<Record> {
        let mut record = Record::new(RecordType::Result);

        // Sequence number
        record.set_field(1, result.metadata.sequence_number.to_string());

        // Test ID
        record.set_field(2, result.test_id.clone());

        // Value
        record.set_field(3, result.value.clone());

        // Units
        if let Some(units) = &result.units {
            record.set_field(4, units.clone());
        }

        // Reference range
        if let Some(range) = &result.reference_range {
            let range_str = match (range.lower_limit, range.upper_limit) {
                (Some(lower), Some(upper)) => format!("{}-{}", lower, upper),
                (Some(lower), None) => format!("{}-", lower),
                (None, Some(upper)) => format!("-{}", upper),
                (None, None) => "".to_string(),
            };
            record.set_field(5, range_str);
        }

        // Flags
        if let Some(flags) = &result.flags {
            if let Some(abnormal_flag) = &flags.abnormal_flag {
                record.set_field(6, abnormal_flag.clone());
            }
        }

        // Sample ID
        record.set_field(12, result.sample_id.clone());

        // Instrument
        if let Some(analyzer) = &result.analyzer_id {
            record.set_field(17, analyzer.clone());
        }

        Ok(record)
    }
}
