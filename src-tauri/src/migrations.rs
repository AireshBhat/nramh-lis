use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_patients_migration() -> Migration {
    Migration {
        version: 1,
        description: "create_patients_table",
        sql: r#"
            CREATE TABLE IF NOT EXISTS patients (
                id TEXT PRIMARY KEY NOT NULL,
                last_name TEXT,
                first_name TEXT,
                middle_name TEXT,
                title TEXT,
                birth_date TEXT,
                sex TEXT NOT NULL CHECK (sex IN ('M', 'F', 'U')),
                street TEXT,
                city TEXT,
                state TEXT,
                zip TEXT,
                country_code TEXT,
                telephone TEXT, -- JSON array of phone numbers
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
            
            -- Create indexes for better query performance
            CREATE INDEX IF NOT EXISTS idx_patients_id ON patients(id);
            CREATE INDEX IF NOT EXISTS idx_patients_last_name ON patients(last_name);
            CREATE INDEX IF NOT EXISTS idx_patients_first_name ON patients(first_name);
            CREATE INDEX IF NOT EXISTS idx_patients_birth_date ON patients(birth_date);
            CREATE INDEX IF NOT EXISTS idx_patients_sex ON patients(sex);
            CREATE INDEX IF NOT EXISTS idx_patients_created_at ON patients(created_at);
        "#,
        kind: MigrationKind::Up,
    }
}

pub fn get_test_results_migration() -> Migration {
    Migration {
        version: 2,
        description: "create_test_results_table",
        sql: r#"
            CREATE TABLE IF NOT EXISTS test_results (
                id TEXT PRIMARY KEY NOT NULL,
                test_id TEXT NOT NULL,
                sample_id TEXT NOT NULL,
                value TEXT NOT NULL,
                units TEXT,
                reference_range_lower REAL,
                reference_range_upper REAL,
                abnormal_flag TEXT,
                nature_of_abnormality TEXT,
                status TEXT NOT NULL CHECK (status IN ('C', 'F', 'P')),
                completed_date_time TEXT,
                sequence_number INTEGER NOT NULL,
                instrument TEXT,
                analyzer_id TEXT,
                patient_id TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY(patient_id) REFERENCES patients(id) ON DELETE RESTRICT ON UPDATE CASCADE
            );
            
            -- Create indexes for better query performance
            CREATE INDEX IF NOT EXISTS idx_test_results_id ON test_results(id);
            CREATE INDEX IF NOT EXISTS idx_test_results_test_id ON test_results(test_id);
            CREATE INDEX IF NOT EXISTS idx_test_results_sample_id ON test_results(sample_id);
            CREATE INDEX IF NOT EXISTS idx_test_results_status ON test_results(status);
            CREATE INDEX IF NOT EXISTS idx_test_results_analyzer_id ON test_results(analyzer_id);
            CREATE INDEX IF NOT EXISTS idx_test_results_completed_date_time ON test_results(completed_date_time);
            CREATE INDEX IF NOT EXISTS idx_test_results_created_at ON test_results(created_at);
            CREATE INDEX IF NOT EXISTS idx_test_results_patient_id ON test_results(patient_id);
        "#,
        kind: MigrationKind::Up,
    }
}

pub fn get_migrations() -> Vec<Migration> {
    vec![get_patients_migration(), get_test_results_migration()]
}
