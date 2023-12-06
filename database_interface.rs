use crate::config::Config;
use crate::data_structurer::StructuredData;
use anyhow::{Result, Context};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Transaction};

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

    pub async fn start_transaction(&self) -> Result<Transaction<Sqlite>, sqlx::Error> {
        let transaction = self.pool.begin().await?;
        Ok(transaction)
    }

    pub async fn store_data(&self, data: &StructuredData, transaction: &mut Transaction<Sqlite>) -> Result<()> {
        for entry in &data.entries {
            sqlx::query("INSERT INTO files (record_number, file_name, file_size, creation_time) VALUES (?, ?, ?, ?)")
                .bind(entry.record_number)
                .bind(entry.file_name)
                .bind(entry.file_size)
                .bind(entry.creation_time)
                .execute(transaction)
                .await
                .context("Failed to insert entry into the database")?;
        }
        Ok(())
    }

    pub async fn commit(&self, transaction: &mut Transaction<Sqlite>) -> Result<()> {
        transaction.commit().await.context("Failed to commit database transaction")?;
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
        let mut transaction = db_interface.start_transaction().await?;
        let structured_data = StructuredData {
            entries: vec![
                DbEntry {
                    record_number: 12345,
                    file_name: "test.txt".to_string(),
                    file_size: 1024,
                    creation_time: "2021-01-01T00:00:00Z".to_string(),
                },
            ],
        };
        db_interface.store_data(&structured_data, &mut transaction).await?;
        db_interface.commit(&mut transaction).await?;
        Ok(())
    }
}