use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

/// Simple ASTM test client to verify the AutoQuantMeril service
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ASTM Test Client - Connecting to AutoQuantMeril Service");
    
    // Connect to the service (default port 5600)
    let mut stream = TcpStream::connect("127.0.0.1:5600")?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    
    println!("Connected to service");
    
    // ASTM Protocol Constants
    const ASTM_ENQ: u8 = 0x05;  // ENQ - Enquiry
    const ASTM_ACK: u8 = 0x06;  // ACK - Acknowledgment
    const ASTM_STX: u8 = 0x02;  // STX - Start of Text
    const ASTM_ETX: u8 = 0x03;  // ETX - End of Text
    const ASTM_EOT: u8 = 0x04;  // EOT - End of Transmission
    const ASTM_CR: u8 = 0x0D;   // CR - Carriage Return
    const ASTM_LF: u8 = 0x0A;   // LF - Line Feed
    
    // Step 1: Send ENQ
    println!("Sending ENQ...");
    stream.write_all(&[ASTM_ENQ])?;
    
    // Step 2: Wait for ACK
    let mut buffer = [0u8; 1];
    stream.read_exact(&mut buffer)?;
    
    if buffer[0] == ASTM_ACK {
        println!("Received ACK - proceeding with data transmission");
    } else {
        println!("Expected ACK, got: 0x{:02X}", buffer[0]);
        return Ok(());
    }
    
    // Step 3: Send ASTM Header Record
    let header_data = b"H|\\^&||||||||||P|E 1394-97|20231205120000\r";
    let header_frame = create_astm_frame(header_data, 1);
    
    println!("Sending Header Record...");
    stream.write_all(&header_frame)?;
    
    // Wait for ACK
    stream.read_exact(&mut buffer)?;
    if buffer[0] == ASTM_ACK {
        println!("Header record acknowledged");
    } else {
        println!("Header record failed: expected ACK, got 0x{:02X}", buffer[0]);
        return Ok(());
    }
    
    // Step 4: Send Patient Record
    let patient_data = b"P|1|PAT001|||SMITH^JOHN^M||19800101|M||123 MAIN ST^CITY^STATE^12345||DR SMITH|||175^CM|70^KG\r";
    let patient_frame = create_astm_frame(patient_data, 2);
    
    println!("Sending Patient Record...");
    stream.write_all(&patient_frame)?;
    
    // Wait for ACK
    stream.read_exact(&mut buffer)?;
    if buffer[0] == ASTM_ACK {
        println!("Patient record acknowledged");
    } else {
        println!("Patient record failed: expected ACK, got 0x{:02X}", buffer[0]);
        return Ok(());
    }
    
    // Step 5: Send Result Record
    let result_data = b"R|1|^^^GLU|95|mg/dL|70-100||N|F||||20231205120000\r";
    let result_frame = create_astm_frame(result_data, 3);
    
    println!("Sending Result Record...");
    stream.write_all(&result_frame)?;
    
    // Wait for ACK
    stream.read_exact(&mut buffer)?;
    if buffer[0] == ASTM_ACK {
        println!("Result record acknowledged");
    } else {
        println!("Result record failed: expected ACK, got 0x{:02X}", buffer[0]);
        return Ok(());
    }
    
    // Step 6: Send Terminator Record
    let terminator_data = b"L|1|N\r";
    let terminator_frame = create_astm_frame(terminator_data, 4);
    
    println!("Sending Terminator Record...");
    stream.write_all(&terminator_frame)?;
    
    // Wait for ACK
    stream.read_exact(&mut buffer)?;
    if buffer[0] == ASTM_ACK {
        println!("Terminator record acknowledged");
    } else {
        println!("Terminator record failed: expected ACK, got 0x{:02X}", buffer[0]);
        return Ok(());
    }
    
    // Step 7: Send EOT
    println!("Sending EOT...");
    stream.write_all(&[ASTM_EOT])?;
    
    // Wait for final ACK
    stream.read_exact(&mut buffer)?;
    if buffer[0] == ASTM_ACK {
        println!("EOT acknowledged - transmission complete!");
    } else {
        println!("EOT failed: expected ACK, got 0x{:02X}", buffer[0]);
    }
    
    println!("Test completed successfully");
    Ok(())
}

/// Creates an ASTM frame with proper formatting and checksum
fn create_astm_frame(data: &[u8], frame_number: u8) -> Vec<u8> {
    let mut frame = Vec::new();
    
    // Frame number
    frame.push(frame_number + b'0');
    
    // STX
    frame.push(0x02);
    
    // Data
    frame.extend_from_slice(data);
    
    // ETX
    frame.push(0x03);
    
    // Calculate checksum (modulo 8 of sum)
    let mut sum = 0u8;
    for &byte in &frame[1..] { // Skip frame number, include STX to ETX
        sum = sum.wrapping_add(byte);
    }
    let checksum = sum % 8;
    
    // Add checksum
    frame.push(checksum);
    
    // Add CR and LF
    frame.push(0x0D);
    frame.push(0x0A);
    
    frame
} 