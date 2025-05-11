use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    vec![
        // Migration 1: Create patients table
        Migration {
            version: 1,
            description: "create_patients_table",
            sql: r#"
                CREATE TABLE IF NOT EXISTS patients (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    patient_id TEXT NOT NULL UNIQUE,
                    first_name TEXT,
                    last_name TEXT,
                    middle_name TEXT,
                    title TEXT,
                    birth_date TEXT,
                    sex TEXT NOT NULL,
                    street TEXT,
                    city TEXT,
                    state TEXT,
                    zip TEXT,
                    country_code TEXT,
                    telephone TEXT,
                    ordering_physician TEXT,
                    attending_physician TEXT,
                    referring_physician TEXT,
                    height_value REAL,
                    height_unit TEXT,
                    weight_value REAL,
                    weight_unit TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
                
                CREATE INDEX IF NOT EXISTS idx_patients_patient_id ON patients (patient_id);
            "#,
            kind: MigrationKind::Up,
        },
        // Migration 2: Create analyzers table
        Migration {
            version: 2,
            description: "create_analyzers_table",
            sql: r#"
                CREATE TABLE IF NOT EXISTS analyzers (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    analyzer_id TEXT NOT NULL UNIQUE,
                    name TEXT NOT NULL,
                    model TEXT NOT NULL,
                    serial_number TEXT,
                    manufacturer TEXT,
                    connection_type TEXT NOT NULL CHECK (connection_type IN ('SERIAL', 'TCP/IP')),
                    ip_address TEXT,
                    port INTEGER,
                    com_port TEXT,
                    baud_rate INTEGER,
                    status TEXT CHECK (status IN ('ACTIVE', 'INACTIVE', 'MAINTENANCE')) DEFAULT 'ACTIVE',
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
            "#,
            kind: MigrationKind::Up,
        },
        // Migration 3: Create test_results table
        Migration {
            version: 3,
            description: "create_test_results_table",
            sql: r#"
                CREATE TABLE IF NOT EXISTS test_results (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    result_id TEXT NOT NULL UNIQUE,
                    test_id TEXT NOT NULL,
                    patient_id TEXT NOT NULL,
                    sample_id TEXT NOT NULL,
                    value TEXT NOT NULL,
                    units TEXT,
                    reference_range_lower REAL,
                    reference_range_upper REAL,
                    abnormal_flag TEXT,
                    nature_of_abnormality TEXT,
                    status TEXT NOT NULL,
                    completed_date_time TEXT,
                    sequence_number INTEGER,
                    instrument TEXT,
                    analyzer_id TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY (patient_id) REFERENCES patients (patient_id),
                    FOREIGN KEY (analyzer_id) REFERENCES analyzers (analyzer_id)
                );
                
                CREATE INDEX IF NOT EXISTS idx_test_results_patient_id ON test_results (patient_id);
                CREATE INDEX IF NOT EXISTS idx_test_results_sample_id ON test_results (sample_id);
                CREATE INDEX IF NOT EXISTS idx_test_results_analyzer_id ON test_results (analyzer_id);
            "#,
            kind: MigrationKind::Up,
        },
        // Migration 4: Create result_upload_status table
        Migration {
            version: 4,
            description: "create_result_upload_status_table",
            sql: r#"
                CREATE TABLE IF NOT EXISTS result_upload_status (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    upload_id TEXT NOT NULL UNIQUE,
                    result_id TEXT NOT NULL,
                    external_system_id TEXT NOT NULL,
                    upload_status TEXT CHECK (upload_status IN ('PENDING', 'UPLOADING', 'UPLOADED', 'FAILED')) DEFAULT 'PENDING',
                    upload_date TEXT,
                    response_code TEXT,
                    response_message TEXT,
                    retry_count INTEGER DEFAULT 0,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY (result_id) REFERENCES test_results (result_id)
                );
                
                CREATE INDEX IF NOT EXISTS idx_result_upload_status_result_id ON result_upload_status (result_id);
                CREATE INDEX IF NOT EXISTS idx_result_upload_status_status ON result_upload_status (upload_status);
            "#,
            kind: MigrationKind::Up,
        },
    ]
}
