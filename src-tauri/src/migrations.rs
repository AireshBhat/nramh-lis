use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_analyzers_migration() -> Migration {
    Migration {
        version: 1,
        description: "create_analyzers_table",
        sql: r#"
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
        "#,
        kind: MigrationKind::Up,
    }
}

pub fn get_migrations() -> Vec<Migration> {
    vec![get_analyzers_migration()]
}
