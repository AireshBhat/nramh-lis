-- Migration: 001_create_analyzers_table.sql
-- Version: 1
-- Description: Create analyzers table with all necessary fields and constraints

CREATE TABLE IF NOT EXISTS analyzers (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    model TEXT NOT NULL,
    serial_number TEXT,
    manufacturer TEXT,
    connection_type TEXT NOT NULL CHECK (connection_type IN ('SERIAL', 'TCP/IP')),
    ip_address TEXT,
    port INTEGER,
    com_port TEXT,
    baud_rate INTEGER,
    protocol TEXT NOT NULL CHECK (protocol IN ('ASTM', 'HL7')),
    status TEXT NOT NULL CHECK (status IN ('ACTIVE', 'INACTIVE', 'MAINTENANCE')),
    activate_on_start BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_analyzers_name ON analyzers(name);
CREATE INDEX IF NOT EXISTS idx_analyzers_status ON analyzers(status);
CREATE INDEX IF NOT EXISTS idx_analyzers_connection_type ON analyzers(connection_type);
CREATE INDEX IF NOT EXISTS idx_analyzers_protocol ON analyzers(protocol); 