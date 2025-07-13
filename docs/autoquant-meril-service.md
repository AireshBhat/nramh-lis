# AutoQuantMeril Service Documentation

## Overview

The AutoQuantMeril Service is a core service in the NRAMH LIS 2 system that handles communication with MERIL AutoQuant analyzers using the ASTM protocol over TCP/IP connections. This service acts as a bridge between the lab machines and the frontend application.

## Architecture

### Service Layer Integration

The AutoQuantMeril service is positioned in the service layer of the application architecture:

```
Frontend Layer → API Layer → Service Layer → Protocol Layer → Transport Layer
                                    ↑
                            AutoQuantMeril Service
```

### Key Components

1. **AutoQuantMerilService**: Main service struct that manages the entire lifecycle
2. **ASTM Protocol Handler**: Processes ASTM E1381-02 and E1394-97 messages
3. **TCP Listener**: Manages network connections from analyzers
4. **Event System**: mpsc channel for real-time communication with frontend
5. **Configuration Management**: JSON-based settings via meril.json store

## Features

### ✅ Implemented Features

- **TCP/IP Server**: Listens for incoming connections from AutoQuant analyzers
- **ASTM Protocol Support**: Handles ENQ/ACK/NAK/EOT communication flow
- **Frame Processing**: Validates checksums and parses ASTM records
- **Event Communication**: Real-time events sent to frontend via mpsc channels
- **Configuration Persistence**: Settings stored in meril.json
- **Connection Management**: Tracks active connections and handles disconnections
- **Error Handling**: Comprehensive error reporting and logging

### 🔄 Data Flow

1. **Service Initialization**:
   ```
   Bootup → Load Configuration → Create Service → Start TCP Listener → Ready
   ```

2. **Connection Flow**:
   ```
   Analyzer Connects → ENQ Received → ACK Sent → Frame Processing → Results → Frontend
   ```

3. **ASTM Message Processing**:
   ```
   Raw Data → Frame Validation → Record Parsing → Event Emission → Frontend Update
   ```

## Configuration

### Default Configuration

The service creates a default configuration if none exists:

```json
{
  "config": {
    "analyzer": {
      "id": "auto-generated-uuid",
      "name": "AutoQuant",
      "model": "200i",
      "manufacturer": "Meril Diagnostics PVT LTD",
      "connection_type": "TcpIp",
      "port": 8080,
      "protocol": "Astm",
      "status": "Inactive",
      "activate_on_start": false
    }
  }
}
```

### Configuration Validation

- **Connection Type**: Must be TCP/IP (Serial not supported)
- **Protocol**: Must be ASTM
- **Port**: Must be between 1-65535
- **IP Address**: Valid IP format if specified

## API Commands

### Frontend Integration

The service exposes several Tauri commands for frontend control:

```typescript
// Get service status
const status = await invoke('get_meril_service_status');

// Update configuration
const result = await invoke('update_meril_config', { analyzer: config });

// Fetch current configuration
const config = await invoke('fetch_meril_config');
```

### Event System

The service emits real-time events to the frontend:

```typescript
// Listen for analyzer connections
await listen('meril:analyzer-connected', (event) => {
  console.log('Analyzer connected:', event.payload);
});

// Listen for lab results
await listen('meril:lab-results', (event) => {
  console.log('Lab results received:', event.payload);
});

// Listen for errors
await listen('meril:error', (event) => {
  console.error('Service error:', event.payload);
});
```

## ASTM Protocol Implementation

### Supported Record Types

- **H**: Message Header Record
- **P**: Patient Information Record  
- **O**: Test Order Record
- **R**: Result Record
- **C**: Comment Record
- **Q**: Request Information Record
- **L**: Message Terminator Record

### Communication Flow

1. **Establishment Phase**:
   ```
   Analyzer → ENQ → LIS
   LIS → ACK → Analyzer
   ```

2. **Transfer Phase**:
   ```
   Analyzer → STX + Frame + ETX/ETB + Checksum + CR + LF → LIS
   LIS → ACK → Analyzer
   ```

3. **Termination Phase**:
   ```
   Analyzer → EOT → LIS
   LIS → ACK → Analyzer
   ```

### Checksum Validation

The service implements ASTM checksum validation:
- Modulo 8 of sum of ASCII values
- From 'FN' to character before ETX/ETB
- Validates against received checksum byte

## Error Handling

### Error Categories

1. **Connection Errors**: Network failures, timeouts
2. **Protocol Errors**: Invalid ASTM messages, checksum failures
3. **Configuration Errors**: Invalid settings, missing required fields
4. **System Errors**: Resource exhaustion, unexpected states

### Error Recovery

- **Automatic Retry**: Failed connections are retried
- **Graceful Degradation**: Service continues with reduced functionality
- **Error Reporting**: All errors are logged and reported to frontend
- **State Recovery**: Service maintains consistent state across errors

## Monitoring & Observability

### Logging

The service provides comprehensive logging:

```rust
log::info!("AutoQuantMeril service started on port {}", port);
log::debug!("Processed ASTM frame: {} - {}", record_type, data);
log::error!("Error processing ASTM data: {}", error);
```

### Metrics

- **Connection Count**: Number of active analyzer connections
- **Message Throughput**: ASTM messages processed per second
- **Error Rate**: Failed operations per time period
- **Service Status**: Running/stopped state

## Testing

### Unit Tests

```bash
# Run service tests
cargo test autoquant_meril

# Run configuration tests
cargo test meril_handler
```

### Integration Testing

1. **Mock Analyzer**: Use a TCP client to simulate analyzer communication
2. **Protocol Testing**: Send valid/invalid ASTM messages
3. **Error Scenarios**: Test timeout, disconnection, invalid data
4. **Frontend Integration**: Verify event emission and command handling

## Future Enhancements

### Planned Features

- **Serial Communication**: Support for RS-232 connections
- **Multiple Analyzers**: Concurrent support for multiple machines
- **Advanced Parsing**: Full ASTM record parsing and validation
- **Database Integration**: Store results in SQLite database
- **HIS Integration**: Forward results to hospital information systems
- **Webhook Support**: Real-time notifications to external systems

### Performance Optimizations

- **Connection Pooling**: Reuse connections for better performance
- **Batch Processing**: Process multiple results in batches
- **Caching**: Cache frequently accessed configuration
- **Async Processing**: Non-blocking result processing

## Troubleshooting

### Common Issues

1. **Port Already in Use**:
   - Check if another service is using the configured port
   - Change port in configuration
   - Restart the service

2. **Connection Timeouts**:
   - Verify network connectivity
   - Check firewall settings
   - Ensure analyzer is configured correctly

3. **Invalid ASTM Messages**:
   - Verify analyzer protocol settings
   - Check message format compliance
   - Review checksum calculation

### Debug Mode

Enable debug logging for detailed troubleshooting:

```rust
log::set_max_level(log::LevelFilter::Debug);
```

## Security Considerations

- **Network Security**: TCP connections are unencrypted by default
- **Input Validation**: All ASTM messages are validated before processing
- **Resource Limits**: Connection limits prevent resource exhaustion
- **Error Handling**: Sensitive information is not exposed in error messages

## Dependencies

### Rust Dependencies

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
tauri = { version = "2.0", features = [] }
tauri-plugin-store = "2.0"
```

### System Requirements

- **Operating System**: Windows, macOS, Linux
- **Network**: TCP/IP connectivity
- **Ports**: Configurable port (default: 8080)
- **Memory**: Minimal memory footprint (~10MB)

## Contributing

### Development Guidelines

1. **Code Style**: Follow Rust conventions and clippy recommendations
2. **Testing**: Write unit tests for all new functionality
3. **Documentation**: Update documentation for API changes
4. **Error Handling**: Implement comprehensive error handling
5. **Logging**: Add appropriate log statements for debugging

### Testing Checklist

- [ ] Service starts and stops correctly
- [ ] Configuration loading and validation
- [ ] TCP connection handling
- [ ] ASTM protocol compliance
- [ ] Event emission to frontend
- [ ] Error handling and recovery
- [ ] Performance under load
- [ ] Cross-platform compatibility 