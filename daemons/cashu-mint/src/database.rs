use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use crate::errors::MintError;

/// Database utilities for the Cashu mint
pub struct Database;

impl Database {
    /// Initialize the database connection pool
    pub async fn init(database_url: &str) -> Result<SqlitePool, MintError> {
        // Create database if it doesn't exist
        if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            tracing::info!("Creating database: {}", database_url);
            Sqlite::create_database(database_url)
                .await
                .map_err(|e| MintError::Database(e))?;
        }

        // Create connection pool
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| MintError::Database(e))?;

        tracing::info!("Connected to database: {}", database_url);
        Ok(pool)
    }

    /// Run database migrations
    pub async fn migrate(pool: &SqlitePool) -> Result<(), MintError> {
        tracing::info!("Running database migrations");
        
        // For now, we'll create tables manually since we don't have migration files
        // In a production system, you'd use sqlx migrate!() macro with migration files
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS keysets (
                id TEXT PRIMARY KEY,
                unit TEXT NOT NULL,
                keys TEXT NOT NULL,
                active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| MintError::Database(e))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS proofs (
                secret TEXT PRIMARY KEY,
                amount INTEGER NOT NULL,
                keyset_id TEXT NOT NULL,
                c TEXT NOT NULL,
                state TEXT NOT NULL DEFAULT 'unspent',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                spent_at TIMESTAMP NULL
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| MintError::Database(e))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS quotes (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                amount INTEGER NOT NULL,
                unit TEXT NOT NULL,
                request TEXT NOT NULL,
                state INTEGER NOT NULL,
                expiry TIMESTAMP NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| MintError::Database(e))?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_proofs_keyset ON proofs(keyset_id);")
            .execute(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_proofs_state ON proofs(state);")
            .execute(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_quotes_kind ON quotes(kind);")
            .execute(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_keysets_unit ON keysets(unit);")
            .execute(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        tracing::info!("Database migrations completed successfully");
        Ok(())
    }

    /// Health check for database connection
    pub async fn health_check(pool: &SqlitePool) -> Result<(), MintError> {
        sqlx::query("SELECT 1")
            .fetch_one(pool)
            .await
            .map_err(|e| MintError::Database(e))?;
        
        Ok(())
    }

    /// Get database statistics
    pub async fn get_stats(pool: &SqlitePool) -> Result<DatabaseStats, MintError> {
        let keysets_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM keysets")
            .fetch_one(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        let proofs_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM proofs")
            .fetch_one(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        let quotes_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quotes")
            .fetch_one(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        let spent_proofs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM proofs WHERE state = 'spent'")
            .fetch_one(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        let active_keysets: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM keysets WHERE active = TRUE")
            .fetch_one(pool)
            .await
            .map_err(|e| MintError::Database(e))?;

        Ok(DatabaseStats {
            keysets_count: keysets_count.0 as u64,
            proofs_count: proofs_count.0 as u64,
            quotes_count: quotes_count.0 as u64,
            spent_proofs: spent_proofs.0 as u64,
            active_keysets: active_keysets.0 as u64,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DatabaseStats {
    pub keysets_count: u64,
    pub proofs_count: u64,
    pub quotes_count: u64,
    pub spent_proofs: u64,
    pub active_keysets: u64,
}