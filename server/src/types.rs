use crate::pager;
use bitcoincore_rpc_json as json;
// use json::bitcoin;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BlockStatsInfo {
    pub avgfee: u32,
    pub avgfeerate: u32,
    pub avgtxsize: u64,
    pub blockhash: String,
    pub feerate_percentiles: Vec<u32>,
    pub height: u32,
    pub ins: u32,
    pub maxfee: u64,
    pub maxfeerate: u64,
    pub maxtxsize: u32,
    pub medianfee: u32,
    pub mediantime: u64,
    pub mediantxsize: u32,
    pub minfee: u64,
    pub minfeerate: u64,
    pub mintxsize: u32,
    pub outs: u32,
    pub subsidy: u64,
    pub swtotal_size: u32,
    pub swtotal_weight: u64,
    pub swtxs: u32,
    pub time: u64,
    pub total_out: u64,
    pub total_size: u64,
    pub total_weight: u64,
    pub totalfee: u64,
    pub txs: u32,
    pub utxo_increase: i32,
    pub utxo_size_inc: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BlockStatsResponse {
    pub result: BlockStatsInfo,
}

#[derive(Clone, Debug, Serialize)]
pub struct Block {
    pub header: json::GetBlockHeaderResult,
    pub stats: BlockStatsInfo,
}

#[derive(Clone, Debug, Serialize)]
pub struct BlocksList {
    pub list: Vec<Block>,
    pub pager: Option<pager::Output>,
}
impl std::default::Default for BlocksList {
    fn default() -> Self {
        Self {
            list: vec![],
            pager: None,
        }
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct TxList {
    pub list: Vec<json::GetRawTransactionResult>,
    pub pager: Option<pager::Output>,
}

impl std::default::Default for TxList {
    fn default() -> Self {
        Self {
            list: vec![],
            pager: None,
        }
    }
}

pub mod response {
    use super::*;
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub enum Address {
        #[serde(rename = "error")]
        Failure(String),
        #[serde(rename = "list")]
        Tx(super::TxList),
    }
    impl Address {
        pub fn is_invalid(&self) -> bool {
            if let Self::Failure(_) = self {
                return true;
            }
            false
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub enum BlockList {
        #[serde(rename = "error")]
        Failure(String),
        #[serde(rename = "blocks")]
        Blocks(super::BlocksList),
    }
    impl BlockList {
        pub fn is_invalid(&self) -> bool {
            if let Self::Failure(_) = self {
                return true;
            }
            false
        }
    }

    /// wrapper of the esponse that can be cached
    #[derive(Clone, Debug, Serialize)]
    pub enum Tx {
        #[serde(rename = "error")]
        Failure(String),
        #[serde(rename = "tx")]
        Tx(json::GetRawTransactionResult),
    }
    impl Tx {
        pub fn is_invalid(&self) -> bool {
            if let Self::Failure(_) = self {
                return true;
            }
            false
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub enum Block {
        #[serde(rename = "error")]
        Failure(String),
        #[serde(rename = "block")]
        Block {
            block: json::GetBlockResult,
            stats: super::BlockStatsInfo,
        },
    }

    impl Block {
        pub fn is_invalid(&self) -> bool {
            if let Self::Failure(_) = self {
                return true;
            }
            false
        }
    }

}
