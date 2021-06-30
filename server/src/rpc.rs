use crate::types::{BlockStatsInfo, BlockStatsResponse};
use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc_json as json;
use cached::proc_macro::cached;
use json::bitcoin;
use serde::Serialize;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

#[derive(Clone, Debug)]
pub struct Client {
    pub rpc_addr: String,
    pub rpc_auth: bitcoincore_rpc::Auth,
}
impl Client {
    pub fn new(rpc_addr: &str, rpc_username: &str, rpc_password: &str) -> Self {
        let rpc_auth = if rpc_username.len() > 0 {
            bitcoincore_rpc::Auth::UserPass(rpc_username.to_string(), rpc_password.to_string())
        } else {
            bitcoincore_rpc::Auth::None
        };
        Self {
            rpc_addr: rpc_addr.to_string(),
            rpc_auth,
        }
    }
    pub fn core_client(&self) -> bitcoincore_rpc::Client {
        bitcoincore_rpc::Client::new(self.rpc_addr.clone(), self.rpc_auth.clone()).unwrap()
    }

    pub fn request(&self) -> ureq::Request {
        let agent: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .build();
        match &self.rpc_auth {
            bitcoincore_rpc::Auth::UserPass(user, pass) => {
                let auth_token = format!("{}:{}", user, pass);
                let auth_hdr = format!("Basic {}", base64::encode(auth_token));
                agent
                    .post(self.rpc_addr.as_str())
                    .set("Authorization", auth_hdr.as_str())
                    .set("Content-Type", "application/json")
            }
            _ => agent
                .post(self.rpc_addr.as_str())
                .set("Content-Type", "application/json"),
        }
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Client) -> bool {
        self.rpc_addr == other.rpc_addr
    }
}
impl Eq for Client {}
impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rpc_addr.hash(state);
    }
}

#[cached(time = 60)]
pub fn get_blockchain_info(rpcclient: Client) -> json::GetBlockchainInfoResult {
    let out = rpcclient.core_client().get_blockchain_info().unwrap();
    tracing::info!("get_blockchain_info: {:?}", out);
    out
}

#[cached(time = 60)]
pub fn get_block_info(rpcclient: Client, hash: bitcoin::BlockHash) -> json::GetBlockResult {
    let out = rpcclient.core_client().get_block_info(&hash).unwrap();
    tracing::info!("get_block_info: {:?}", out);
    out
}

/// wrapper of the esponse that can be cached
#[derive(Clone, Debug, Serialize)]
pub enum RawTransactionResponse {
    #[serde(rename="error")]
    Failure(String),
    #[serde(rename="tx")]
    Tx(json::GetRawTransactionResult),
}

#[cached(time = 60)]
pub fn get_raw_transaction_info(
    rpcclient: Client,
    hash: bitcoin::Txid,
) -> RawTransactionResponse {
    let out = rpcclient
        .core_client()
        .get_raw_transaction_info(&hash, None);
    tracing::info!("get_raw_transaction_info: {:?}", out);
    match out {
        Ok(res) => RawTransactionResponse::Tx(res),
        Err(e) => RawTransactionResponse::Failure(e.to_string()),
    }
}

/// this method is not in the library yet
#[cached(time = 60)]
pub fn get_block_stats(rpcclient: Client, hash: bitcoin::BlockHash) -> BlockStatsInfo {
    let payload = format!(
        "{{\"jsonrpc\":\"1.0\",\"id\":\"{}\",\"method\":\"getblockstats\",\"params\":[\"{}\"]}}",
        hash, hash,
    );
    let stats_str: String = rpcclient
        .request()
        .send_string(payload.as_str())
        .unwrap()
        .into_string()
        .unwrap();
    let response: BlockStatsResponse = match serde_json::from_str(stats_str.as_str()) {
        Ok(x) => x,
        Err(e) => {
            tracing::info!("REQUEST >> {}", payload);
            tracing::info!("RESPONSE {}", stats_str);
            tracing::error!("{}", e);
            BlockStatsResponse::default()
        }
    };
    tracing::info!("get_block_stats: {:?}", response.result);
    response.result
}
