use serde::{Deserialize, Serialize};
use std::fmt;

/// Chain identifier - unique identifier for each blockchain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainId {
    #[serde(rename = "eth")]
    Ethereum,
    #[serde(rename = "sol")]
    Solana,
    #[serde(rename = "btc")]
    Bitcoin,
    #[serde(rename = "sui")]
    Sui,
}

impl ChainId {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "eth" | "ethereum" => Ok(ChainId::Ethereum),
            "sol" | "solana" => Ok(ChainId::Solana),
            "btc" | "bitcoin" => Ok(ChainId::Bitcoin),
            "sui" => Ok(ChainId::Sui),
            _ => Err(anyhow::anyhow!("Unsupported chain: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ChainId::Ethereum => "eth",
            ChainId::Solana => "sol",
            ChainId::Bitcoin => "btc",
            ChainId::Sui => "sui",
        }
    }
}

impl fmt::Display for ChainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Chain-agnostic address representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    pub chain_id: ChainId,
    pub value: String,
}

impl Address {
    pub fn new(chain_id: ChainId, value: String) -> Self {
        Self { chain_id, value }
    }

    pub fn from_str(chain_id: ChainId, s: &str) -> Self {
        Self {
            chain_id,
            value: s.to_string(),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.chain_id, self.value)
    }
}

/// Balance representation (as string to handle large numbers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub amount: String,
    pub decimals: u8,
    pub symbol: String,
}

impl Balance {
    pub fn new(amount: String, decimals: u8, symbol: String) -> Self {
        Self {
            amount,
            decimals,
            symbol,
        }
    }
}

/// Transaction request - input for building a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxRequest {
    pub from: Address,
    pub to: Address,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
}

/// Unsigned transaction - ready to be signed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsignedTx {
    pub chain_id: ChainId,
    pub raw_data: Vec<u8>,
    pub tx_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Signed transaction - ready to be sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTx {
    pub chain_id: ChainId,
    pub raw_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub tx_type: String,
}

/// Transaction hash
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxHash {
    pub chain_id: ChainId,
    pub value: String,
}

impl TxHash {
    pub fn new(chain_id: ChainId, value: String) -> Self {
        Self { chain_id, value }
    }
}

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.chain_id, self.value)
    }
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TxStatus {
    Pending,
    Confirmed,
    Failed,
    NotFound,
}

/// Transaction status with details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxStatusInfo {
    pub hash: TxHash,
    pub status: TxStatus,
    pub block_number: Option<u64>,
    pub confirmations: Option<u64>,
    pub error: Option<String>,
}

/// Cryptographic key abstraction
#[async_trait::async_trait]
pub trait Key: Send + Sync {
    /// Get the public address for this key
    fn address(&self, chain_id: ChainId) -> anyhow::Result<Address>;
    
    /// Sign data with this key
    async fn sign(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
    
    /// Get the chain IDs this key supports
    fn supported_chains(&self) -> Vec<ChainId>;
}

pub mod key;

