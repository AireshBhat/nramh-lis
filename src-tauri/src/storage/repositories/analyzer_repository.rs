use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use chrono::{DateTime, Utc};

use crate::model::{Analyzer, AnalyzerStatus, ConnectionType, Protocol};
use crate::storage::traits::AnalyzerRepository;
use crate::model::Error as ModelError;

#[derive(Debug)]
pub struct SqliteAnalyzerRepository {
    pool: SqlitePool,
}

impl SqliteAnalyzerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl crate::storage::traits::Repository<Analyzer> for SqliteAnalyzerRepository {
    type Error = ModelError;
    type Id = String;

    async fn create(&self, analyzer: &Analyzer) -> Result<Self::Id, Self::Error> {
        let query = r#"
            INSERT INTO analyzers (
                id, name, model, serial_number, manufacturer, connection_type,
                ip_address, port, com_port, baud_rate, protocol, status,
                activate_on_start, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(query)
            .bind(&analyzer.id)
            .bind(&analyzer.name)
            .bind(&analyzer.model)
            .bind(&analyzer.serial_number)
            .bind(&analyzer.manufacturer)
            .bind(analyzer.connection_type.to_string())
            .bind(&analyzer.ip_address)
            .bind(analyzer.port)
            .bind(&analyzer.com_port)
            .bind(analyzer.baud_rate)
            .bind(analyzer.protocol.to_string())
            .bind(analyzer.status.to_string())
            .bind(analyzer.activate_on_start)
            .bind(analyzer.created_at)
            .bind(analyzer.updated_at)
            .execute(&self.pool)
            .await?;

        Ok(analyzer.id.clone())
    }

    async fn find_by_id(&self, id: Self::Id) -> Result<Option<Analyzer>, Self::Error> {
        let query = "SELECT * FROM analyzers WHERE id = ?";
        
        let row = sqlx::query(query)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.row_to_analyzer(row)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, id: Self::Id, analyzer: &Analyzer) -> Result<(), Self::Error> {
        let query = r#"
            UPDATE analyzers SET
                name = ?, model = ?, serial_number = ?, manufacturer = ?,
                connection_type = ?, ip_address = ?, port = ?, com_port = ?,
                baud_rate = ?, protocol = ?, status = ?, activate_on_start = ?,
                updated_at = ?
            WHERE id = ?
        "#;

        sqlx::query(query)
            .bind(&analyzer.name)
            .bind(&analyzer.model)
            .bind(&analyzer.serial_number)
            .bind(&analyzer.manufacturer)
            .bind(analyzer.connection_type.to_string())
            .bind(&analyzer.ip_address)
            .bind(analyzer.port)
            .bind(&analyzer.com_port)
            .bind(analyzer.baud_rate)
            .bind(analyzer.protocol.to_string())
            .bind(analyzer.status.to_string())
            .bind(analyzer.activate_on_start)
            .bind(analyzer.updated_at)
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Self::Id) -> Result<(), Self::Error> {
        let query = "DELETE FROM analyzers WHERE id = ?";
        
        sqlx::query(query)
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Analyzer>, Self::Error> {
        let mut query = "SELECT * FROM analyzers ORDER BY name".to_string();
        
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await?;

        let mut analyzers = Vec::new();
        for row in rows {
            analyzers.push(self.row_to_analyzer(row)?);
        }

        Ok(analyzers)
    }
}

#[async_trait]
impl AnalyzerRepository for SqliteAnalyzerRepository {
    async fn find_by_status(&self, status: AnalyzerStatus) -> Result<Vec<Analyzer>, Self::Error> {
        let query = "SELECT * FROM analyzers WHERE status = ? ORDER BY name";
        
        let rows = sqlx::query(query)
            .bind(status.to_string())
            .fetch_all(&self.pool)
            .await?;

        let mut analyzers = Vec::new();
        for row in rows {
            analyzers.push(self.row_to_analyzer(row)?);
        }

        Ok(analyzers)
    }

    async fn find_by_connection_type(&self, connection_type: &str) -> Result<Vec<Analyzer>, Self::Error> {
        let query = "SELECT * FROM analyzers WHERE connection_type = ? ORDER BY name";
        
        let rows = sqlx::query(query)
            .bind(connection_type)
            .fetch_all(&self.pool)
            .await?;

        let mut analyzers = Vec::new();
        for row in rows {
            analyzers.push(self.row_to_analyzer(row)?);
        }

        Ok(analyzers)
    }

    async fn find_active_analyzers(&self) -> Result<Vec<Analyzer>, Self::Error> {
        self.find_by_status(AnalyzerStatus::Active).await
    }

    async fn update_status(&self, id: &str, status: AnalyzerStatus) -> Result<(), Self::Error> {
        let query = "UPDATE analyzers SET status = ?, updated_at = ? WHERE id = ?";
        
        sqlx::query(query)
            .bind(status.to_string())
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_serial_number(&self, serial_number: &str) -> Result<Option<Analyzer>, Self::Error> {
        let query = "SELECT * FROM analyzers WHERE serial_number = ?";
        
        let row = sqlx::query(query)
            .bind(serial_number)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.row_to_analyzer(row)?)),
            None => Ok(None),
        }
    }
}

impl SqliteAnalyzerRepository {
    fn row_to_analyzer(&self, row: sqlx::sqlite::SqliteRow) -> Result<Analyzer, ModelError> {
        Ok(Analyzer {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            model: row.try_get("model")?,
            serial_number: row.try_get("serial_number")?,
            manufacturer: row.try_get("manufacturer")?,
            connection_type: ConnectionType::from(row.try_get::<String, _>("connection_type")?.as_str()),
            ip_address: row.try_get("ip_address")?,
            port: row.try_get("port")?,
            com_port: row.try_get("com_port")?,
            baud_rate: row.try_get("baud_rate")?,
            protocol: Protocol::from(row.try_get::<String, _>("protocol")?.as_str()),
            status: AnalyzerStatus::from(row.try_get::<String, _>("status")?.as_str()),
            activate_on_start: row.try_get("activate_on_start")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
} 