use async_trait::async_trait;
use cdk::{
    mint::MintStorage,
    nuts::{Id, KeySet, Keys, MintQuoteState, Proof, ProofState},
    types::{QuoteId, QuoteKind},
    Amount,
};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Simple in-memory storage for testing - implements CDK 0.11.0 MintStorage trait
#[derive(Clone)]
pub struct SimpleStorage {
    keysets: RwLock<HashMap<Id, KeySet>>,
    keys: RwLock<HashMap<Id, Keys>>,
    quotes: RwLock<HashMap<QuoteId, SimpleQuote>>,
    proofs: RwLock<HashMap<String, Proof>>,
}

#[derive(Clone, Debug)]
pub struct SimpleQuote {
    pub id: QuoteId,
    pub amount: Amount,
    pub state: MintQuoteState,
    pub kind: QuoteKind,
}

impl SimpleStorage {
    pub fn new() -> Self {
        Self {
            keysets: RwLock::new(HashMap::new()),
            keys: RwLock::new(HashMap::new()),
            quotes: RwLock::new(HashMap::new()),
            proofs: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MintStorage for SimpleStorage {
    type Err = cdk::Error;

    async fn add_keyset(&self, keyset: KeySet) -> Result<(), Self::Err> {
        let mut keysets = self.keysets.write().await;
        keysets.insert(keyset.id, keyset);
        Ok(())
    }

    async fn get_keyset(&self, id: &Id) -> Result<Option<KeySet>, Self::Err> {
        let keysets = self.keysets.read().await;
        Ok(keysets.get(id).cloned())
    }

    async fn get_keysets(&self) -> Result<Vec<KeySet>, Self::Err> {
        let keysets = self.keysets.read().await;
        Ok(keysets.values().cloned().collect())
    }

    async fn add_keys(&self, keys: Keys) -> Result<(), Self::Err> {
        let mut all_keys = self.keys.write().await;
        all_keys.insert(keys.id, keys);
        Ok(())
    }

    async fn get_keys(&self, id: &Id) -> Result<Option<Keys>, Self::Err> {
        let keys = self.keys.read().await;
        Ok(keys.get(id).cloned())
    }

    async fn get_mint_quote(&self, quote_id: &QuoteId) -> Result<Option<cdk::mint::MintQuote>, Self::Err> {
        let quotes = self.quotes.read().await;
        if let Some(quote) = quotes.get(quote_id) {
            // Convert SimpleQuote to MintQuote
            Ok(Some(cdk::mint::MintQuote {
                id: quote.id.clone(),
                amount: quote.amount,
                state: quote.state,
                request: "".to_string(), // Stub for testing
                unit: cdk::nuts::CurrencyUnit::Sat,
                expiry: None,
                single_use: false,
            }))
        } else {
            Ok(None)
        }
    }

    async fn add_mint_quote(&self, quote: cdk::mint::MintQuote) -> Result<(), Self::Err> {
        let mut quotes = self.quotes.write().await;
        let simple_quote = SimpleQuote {
            id: quote.id.clone(),
            amount: quote.amount,
            state: quote.state,
            kind: QuoteKind::Bolt11,
        };
        quotes.insert(quote.id, simple_quote);
        Ok(())
    }

    async fn update_mint_quote_state(
        &self,
        quote_id: &QuoteId,
        state: MintQuoteState,
    ) -> Result<(), Self::Err> {
        let mut quotes = self.quotes.write().await;
        if let Some(quote) = quotes.get_mut(quote_id) {
            quote.state = state;
        }
        Ok(())
    }

    async fn get_melt_quote(&self, _quote_id: &QuoteId) -> Result<Option<cdk::mint::MeltQuote>, Self::Err> {
        // Stub implementation for testing
        Ok(None)
    }

    async fn add_melt_quote(&self, _quote: cdk::mint::MeltQuote) -> Result<(), Self::Err> {
        // Stub implementation for testing
        Ok(())
    }

    async fn update_melt_quote_state(
        &self,
        _quote_id: &QuoteId,
        _state: cdk::nuts::MeltQuoteState,
    ) -> Result<(), Self::Err> {
        // Stub implementation for testing
        Ok(())
    }

    async fn add_proofs(&self, proofs: Vec<Proof>) -> Result<(), Self::Err> {
        let mut all_proofs = self.proofs.write().await;
        for proof in proofs {
            all_proofs.insert(proof.secret.clone(), proof);
        }
        Ok(())
    }

    async fn get_proofs_by_secret(&self, secrets: &[String]) -> Result<Vec<Proof>, Self::Err> {
        let proofs = self.proofs.read().await;
        let mut result = Vec::new();
        for secret in secrets {
            if let Some(proof) = proofs.get(secret) {
                result.push(proof.clone());
            }
        }
        Ok(result)
    }

    async fn update_proof_state(&self, secret: &str, state: ProofState) -> Result<(), Self::Err> {
        let mut proofs = self.proofs.write().await;
        if let Some(proof) = proofs.get_mut(secret) {
            proof.state = state;
        }
        Ok(())
    }

    async fn get_proof_state(&self, secret: &str) -> Result<Option<ProofState>, Self::Err> {
        let proofs = self.proofs.read().await;
        Ok(proofs.get(secret).map(|p| p.state))
    }
}