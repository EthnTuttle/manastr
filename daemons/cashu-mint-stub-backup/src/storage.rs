use async_trait::async_trait;
use cdk::{
    mint::MintStorage,
    nuts::{Id, KeySet, Keys, MintQuoteState, Proof, ProofState},
    types::{QuoteId, QuoteKind},
    Amount,
};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use crate::errors::MintError;

/// SQLite-based storage implementation for the Cashu mint
pub struct SqliteMintStorage {
    pool: SqlitePool,
}

impl SqliteMintStorage {
    pub async fn new(pool: SqlitePool) -> Result<Self, MintError> {
        Ok(Self { pool })
    }

    /// Initialize database tables if they don't exist
    pub async fn init_tables(&self) -> Result<(), MintError> {
        // This would normally be handled by migrations, but including here for completeness
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
        .execute(&self.pool)
        .await?;

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
        .execute(&self.pool)
        .await?;

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
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl MintStorage for SqliteMintStorage {
    type Err = MintError;

    async fn add_active_keyset(&self, keyset: &KeySet) -> Result<(), Self::Err> {
        let keys_json = serde_json::to_string(&keyset.keys)
            .map_err(|e| MintError::Serialization(e))?;

        sqlx::query(
            "INSERT OR REPLACE INTO keysets (id, unit, keys, active) VALUES (?, ?, ?, ?)"
        )
        .bind(&keyset.id.to_string())
        .bind(&keyset.unit.to_string())
        .bind(&keys_json)
        .bind(true)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_active_keyset_by_unit(&self, unit: &cdk::nuts::CurrencyUnit) -> Result<Option<KeySet>, Self::Err> {
        let row = sqlx::query(
            "SELECT id, unit, keys FROM keysets WHERE unit = ? AND active = TRUE ORDER BY created_at DESC LIMIT 1"
        )
        .bind(&unit.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let unit_str: String = row.get("unit");
            let keys_json: String = row.get("keys");

            let keys: Keys = serde_json::from_str(&keys_json)
                .map_err(|e| MintError::Serialization(e))?;

            let unit = unit_str.parse()
                .map_err(|_| MintError::Config(format!("Invalid unit: {}", unit_str)))?;

            Ok(Some(KeySet {
                id: Id::from_hex(&id).map_err(|_| MintError::Config(format!("Invalid keyset ID: {}", id)))?,
                unit,
                keys,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_keyset(&self, keyset_id: &Id) -> Result<Option<KeySet>, Self::Err> {
        let row = sqlx::query(
            "SELECT id, unit, keys FROM keysets WHERE id = ?"
        )
        .bind(&keyset_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let unit_str: String = row.get("unit");
            let keys_json: String = row.get("keys");

            let keys: Keys = serde_json::from_str(&keys_json)
                .map_err(|e| MintError::Serialization(e))?;

            let unit = unit_str.parse()
                .map_err(|_| MintError::Config(format!("Invalid unit: {}", unit_str)))?;

            Ok(Some(KeySet {
                id: *keyset_id,
                unit,
                keys,
            }))
        } else {
            Ok(None)
        }
    }

    async fn add_mint_quote(&self, quote: &cdk::nuts::MintQuoteBolt11) -> Result<(), Self::Err> {
        sqlx::query(
            "INSERT INTO quotes (id, kind, amount, unit, request, state, expiry) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&quote.id)
        .bind("mint")
        .bind(quote.amount.0 as i64)
        .bind(&quote.unit.to_string())
        .bind(&quote.request)
        .bind(quote.state as i32)
        .bind(quote.expiry.map(|e| chrono::DateTime::from_timestamp(e, 0)))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_mint_quote(&self, quote_id: &QuoteId) -> Result<Option<cdk::nuts::MintQuoteBolt11>, Self::Err> {
        let row = sqlx::query(
            "SELECT id, amount, unit, request, state, expiry FROM quotes WHERE id = ? AND kind = 'mint'"
        )
        .bind(quote_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let amount: i64 = row.get("amount");
            let unit_str: String = row.get("unit");
            let request: String = row.get("request");
            let state: i32 = row.get("state");
            let expiry: Option<chrono::DateTime<chrono::Utc>> = row.get("expiry");

            let unit = unit_str.parse()
                .map_err(|_| MintError::Config(format!("Invalid unit: {}", unit_str)))?;

            let state = match state {
                0 => MintQuoteState::Unpaid,
                1 => MintQuoteState::Paid,
                2 => MintQuoteState::Issued,
                _ => return Err(MintError::Config(format!("Invalid quote state: {}", state))),
            };

            Ok(Some(cdk::nuts::MintQuoteBolt11 {
                id: quote_id.clone(),
                amount: Amount(amount as u64),
                unit,
                request,
                state,
                expiry: expiry.map(|e| e.timestamp()),
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_mint_quote_state(
        &self,
        quote_id: &QuoteId,
        state: MintQuoteState,
    ) -> Result<(), Self::Err> {
        sqlx::query("UPDATE quotes SET state = ? WHERE id = ? AND kind = 'mint'")
            .bind(state as i32)
            .bind(quote_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn add_melt_quote(&self, quote: &cdk::nuts::MeltQuoteBolt11) -> Result<(), Self::Err> {
        sqlx::query(
            "INSERT INTO quotes (id, kind, amount, unit, request, state, expiry) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&quote.id)
        .bind("melt")
        .bind(quote.amount.0 as i64)
        .bind(&quote.unit.to_string())
        .bind(&quote.request)
        .bind(quote.state as i32)
        .bind(quote.expiry.map(|e| chrono::DateTime::from_timestamp(e, 0)))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_melt_quote(&self, quote_id: &QuoteId) -> Result<Option<cdk::nuts::MeltQuoteBolt11>, Self::Err> {
        let row = sqlx::query(
            "SELECT id, amount, unit, request, state, expiry FROM quotes WHERE id = ? AND kind = 'melt'"
        )
        .bind(quote_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let amount: i64 = row.get("amount");
            let unit_str: String = row.get("unit");
            let request: String = row.get("request");
            let state: i32 = row.get("state");
            let expiry: Option<chrono::DateTime<chrono::Utc>> = row.get("expiry");

            let unit = unit_str.parse()
                .map_err(|_| MintError::Config(format!("Invalid unit: {}", unit_str)))?;

            let state = match state {
                0 => cdk::nuts::MeltQuoteState::Unpaid,
                1 => cdk::nuts::MeltQuoteState::Paid,
                _ => return Err(MintError::Config(format!("Invalid melt quote state: {}", state))),
            };

            Ok(Some(cdk::nuts::MeltQuoteBolt11 {
                id: quote_id.clone(),
                amount: Amount(amount as u64),
                unit,
                request,
                state,
                expiry: expiry.map(|e| e.timestamp()),
                fee_reserve: Amount(0), // Default fee reserve
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_melt_quote_state(
        &self,
        quote_id: &QuoteId,
        state: cdk::nuts::MeltQuoteState,
    ) -> Result<(), Self::Err> {
        sqlx::query("UPDATE quotes SET state = ? WHERE id = ? AND kind = 'melt'")
            .bind(state as i32)
            .bind(quote_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn add_proofs(&self, proofs: &[Proof]) -> Result<(), Self::Err> {
        for proof in proofs {
            sqlx::query(
                "INSERT OR REPLACE INTO proofs (secret, amount, keyset_id, c, state) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&proof.secret)
            .bind(proof.amount.0 as i64)
            .bind(&proof.keyset_id.to_string())
            .bind(&proof.c)
            .bind("spent")
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn get_proof_state(&self, secret: &str) -> Result<Option<ProofState>, Self::Err> {
        let row = sqlx::query("SELECT state FROM proofs WHERE secret = ?")
            .bind(secret)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let state_str: String = row.get("state");
            let state = match state_str.as_str() {
                "unspent" => ProofState::Unspent,
                "pending" => ProofState::Pending,
                "spent" => ProofState::Spent,
                _ => return Err(MintError::Config(format!("Invalid proof state: {}", state_str))),
            };
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    async fn set_proof_state(&self, secret: &str, state: ProofState) -> Result<(), Self::Err> {
        let state_str = match state {
            ProofState::Unspent => "unspent",
            ProofState::Pending => "pending",
            ProofState::Spent => "spent",
        };

        sqlx::query("UPDATE proofs SET state = ? WHERE secret = ?")
            .bind(state_str)
            .bind(secret)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}