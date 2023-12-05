use crate::config::Config;
use crate::data_structurer::StructuredData;
use anyhow::{Result, Context};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct DatabaseInterface {
    pool: Pool<Sqlite>,
}

impl DatabaseInterface {
    pub async fn new(config: &Config) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await
            .context("Failed to connect to the database")?;

        Ok(DatabaseInterface { pool })
    }

    pub async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                record_number INTEGER PRIMARY KEY,
                -- Add more columns as necessary to store the file information
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create tables")?;

        Ok(())
    }

    pub async fn insert_entries(&self, data: &StructuredData) -> Result<()> {
        for entry in &data.entries {
            sqlx::query(
                r#"
                INSERT INTO files (record_number /*, other columns */)
                VALUES (? /*, other values */)
                "#,
            )
            .bind(entry.record_number)
            // Bind other values as necessary
            .execute(&self.pool)
            .await
            .context("Failed to insert entry into the database")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::data_structurer::{DbEntry, StructuredData};

    #[tokio::test]
    async fn test_database_operations() -> Result<()> {
        let config = Config::new()?;
        let db_interface = DatabaseInterface::new(&config).await?;
        db_interface.create_tables().await?;

        let structured_data = StructuredData {
            entries: vec![
                DbEntry {
                    record_number: 12345,
                    // Initialize other fields as necessary
                },
                // Add more entries if needed
            ],
        };

        db_interface.insert_entries(&structured_data).await?;

        // Add more tests to verify database operations if necessary

        Ok(())
    }
}
