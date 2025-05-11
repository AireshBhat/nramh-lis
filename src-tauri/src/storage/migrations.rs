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
            "#,
            kind: MigrationKind::Up,
        },
        // Migration 2: Create test_results table
        Migration {
            version: 2,
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
                    is_uploaded BOOLEAN NOT NULL DEFAULT false,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY (patient_id) REFERENCES patients (patient_id)
                );
            "#,
            kind: MigrationKind::Up,
        },
        // Migration 3: Add indexes for better query performance
        Migration {
            version: 3,
            description: "add_indexes",
            sql: r#"
                CREATE INDEX IF NOT EXISTS idx_patients_patient_id ON patients (patient_id);
                CREATE INDEX IF NOT EXISTS idx_test_results_patient_id ON test_results (patient_id);
                CREATE INDEX IF NOT EXISTS idx_test_results_sample_id ON test_results (sample_id);
                CREATE INDEX IF NOT EXISTS idx_test_results_is_uploaded ON test_results (is_uploaded);
            "#,
            kind: MigrationKind::Up,
        },
    ]
}
