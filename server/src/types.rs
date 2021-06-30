use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PagingInput {
    // token to start
    pub from: Option<String>,
    // limit of records
    pub limit: u32,
}

impl Default for PagingInput {
    fn default() -> Self {
        Self {
            from: None,
            limit: 20,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PagingOutput {
    // token to start the next page
    pub from: Option<String>,
}

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

#[derive(Clone, Debug, Default, Deserialize)]
pub struct BlockStatsResponse {
    pub result: BlockStatsInfo,
}
