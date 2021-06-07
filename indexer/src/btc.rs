use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

pub trait BlockchainClient {
    fn get_chain_info(&self) -> anyhow::Result<ChainInfo>;
    fn get_block(&self, hash: &str) -> anyhow::Result<BlockInfoCombined>;
}

struct Client {
    address: String,
    username: String,
    password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfo {
    pub chain: String,
    pub blocks: u32,
    pub headers: u32,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: u64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct ChainInfoResponse {
    result: ChainInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxScriptPubKey {
    pub asm: String,
    pub hex: String,
    pub req_sigs: Option<u32>,
    #[serde(rename = "type")]
    pub script_type: String, // witness_v0_keyhash, witness_v0_scripthash, pubkeyhash, nulldata
    pub addresses: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTxVin {
    pub txid: Option<String>,
    pub vout: Option<u32>,
    pub script_sig: Option<TxScriptSig>,
    pub txinwitness: Option<Vec<String>>,
    pub sequence: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTxVout {
    pub value: f32,
    pub n: u32,
    pub script_pub_key: TxScriptPubKey,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransaction {
    pub txid: Option<String>,
    pub hash: String,
    pub version: u32,
    pub size: u32,
    pub vsize: u32,
    pub weight: u32,
    pub locktime: u32,
    pub vin: Vec<BlockTxVin>,
    pub vout: Vec<BlockTxVout>,
    pub hex: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub hash: String,
    pub confirmations: i32,
    pub strippedsize: u64,
    pub size: i64,
    pub weight: u64,
    pub height: u32,
    pub version: u64,
    pub version_hex: String,
    pub merkleroot: String,
    pub tx: Vec<BlockTransaction>,
    pub time: i64,
    pub mediantime: i64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u32,
    pub previousblockhash: String,
    pub nextblockhash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockStatsInfo {
    pub avgfee: u32,
    pub avgfeerate: u32,
    pub avgtxsize: u64,
    pub blockhash: String,
    pub feerate_percentiles: Vec<u32>,
    pub height: u32,
    pub ins: u32,
    pub maxfee: u32,
    pub maxfeerate: u32,
    pub maxtxsize: u32,
    pub medianfee: u32,
    pub mediantime: u64,
    pub mediantxsize: u32,
    pub minfee: u32,
    pub minfeerate: u32,
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

#[derive(Clone, Debug, Deserialize)]
struct BlockStatsResponse {
    result: BlockStatsInfo,
}

#[derive(Clone, Debug, Deserialize)]
struct BlockInfoResponse {
    result: BlockInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockInfoCombined {
    pub info: BlockInfo,
    pub stats: BlockStatsInfo,
}

impl BlockchainClient for Client {
    fn get_chain_info(&self) -> anyhow::Result<ChainInfo> {
        let agent: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .build();
        let auth_token = format!("{}:{}", self.username.as_str(), self.password.as_str());
        let result: ChainInfoResponse = agent.post(self.address.as_str())
            .set("Authorization", format!("Basic {}", base64::encode(auth_token)).as_str())
            .set("Content-Type", "application/json")
            .send_string("{\"jsonrpc\":\"1.0\",\"id\":\"i0\",\"method\":\"getblockchaininfo\",\"params\":[]}")?
            .into_json()?;
        println!("{:?}", result);
        Ok(result.result)
    }

    fn get_block(&self, hash: &str) -> anyhow::Result<BlockInfoCombined> {
        let agent: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .build();
        let auth_token = format!("{}:{}", self.username.as_str(), self.password.as_str());
        let auth_hdr = format!("Basic {}", base64::encode(auth_token));

        let payload1 = format!(
            "{{\"jsonrpc\":\"1.0\",\"id\":\"{}\",\"method\":\"getblockstats\",\"params\":[\"{}\"]}}",
            hash, hash,
        );
        let stats: BlockStatsResponse = agent
            .post(self.address.as_str())
            .set("Authorization", auth_hdr.as_str())
            .set("Content-Type", "application/json")
            .send_string(payload1.as_str())?
            .into_json()?;

        let payload = format!(
            "{{\"jsonrpc\":\"1.0\",\"id\":\"{}\",\"method\":\"getblock\",\"params\":[\"{}\",2]}}",
            hash, hash
        );
        let info: BlockInfoResponse = agent
            .post(self.address.as_str())
            .set("Authorization", auth_hdr.as_str())
            .set("Content-Type", "application/json")
            .send_string(payload.as_str())?
            .into_json()?;
        Ok(BlockInfoCombined {
            stats: stats.result,
            info: info.result,
        })
    }
}

pub fn new() -> Box<dyn BlockchainClient> {
    Box::new(Client {
        address: "http://localhost:8332/".to_owned(),
        username: "bitcoinrpc".to_owned(),
        password: "bitcoinpass".to_owned(),
    })
}
