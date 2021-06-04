use serde::Deserialize;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

pub trait BlockchainClient {
    fn get_chain_info(&self) -> anyhow::Result<ChainInfo>;
    fn get_block(&self, hash: String) -> anyhow::Result<BlockInfo>;
}

struct Client {
    address: String,
    username: String,
    password: String,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxScriptPubKey {
    pub asm: String,
    pub hex: String,
    pub req_sigs: Option<u32>,
    #[serde(rename = "type")]
    pub script_type: String, // witness_v0_keyhash, witness_v0_scripthash, pubkeyhash, nulldata
    pub addresses: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTxVin {
    pub txid: Option<String>,
    pub vout: Option<u32>,
    pub script_sig: Option<TxScriptSig>,
    pub txinwitness: Option<Vec<String>>,
    pub sequence: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTxVout {
    pub value: f32,
    pub n: u32,
    pub script_pub_key: TxScriptPubKey,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub hash: String,
    pub confirmations: i32,
    pub strippedsize: u64,
    pub size: u64,
    pub weight: u64,
    pub height: u32,
    pub version: u64,
    pub version_hex: String,
    pub merkleroot: String,
    pub tx: Vec<BlockTransaction>,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u32,
    pub previousblockhash: String,
    pub nextblockhash: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct BlockInfoResponse {
    result: BlockInfo,
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

    fn get_block(&self, hash: String) -> anyhow::Result<BlockInfo> {
        let agent: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .build();
        let auth_token = format!("{}:{}", self.username.as_str(), self.password.as_str());
        let payload = format!(
            "{{\"jsonrpc\":\"1.0\",\"id\":\"{}\",\"method\":\"getblock\",\"params\":[\"{}\",2]}}",
            hash.clone(), hash.clone());
        let result: BlockInfoResponse = agent
            .post(self.address.as_str())
            .set(
                "Authorization",
                format!("Basic {}", base64::encode(auth_token)).as_str(),
            )
            .set("Content-Type", "application/json")
            .send_string(payload.as_str())?
            .into_json()?;
        Ok(result.result)
    }
}

pub fn new() -> Box<dyn BlockchainClient> {
    Box::new(Client {
        address: "http://localhost:8332/".to_owned(),
        username: "bitcoinrpc".to_owned(),
        password: "bitcoinpass".to_owned(),
    })
}
