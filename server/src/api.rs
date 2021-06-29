use crate::State;
use serde::{Deserialize, Serialize};
// use chrono::prelude::*;
// use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::collections::HashMap;
use tide::{Body, Request, Response, Result};

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

mod rpc {
    use super::{BlockStatsInfo, BlockStatsResponse};
    use bitcoincore_rpc::RpcApi;
    use bitcoincore_rpc_json as json;
    use cached::proc_macro::cached;
    use json::bitcoin;
    use std::time::Duration;
    use ureq::{Agent, AgentBuilder};

    #[cached(time = 60)]
    pub fn get_blockchain_info(
        rpc_addr: String,
        rpc_auth: bitcoincore_rpc::Auth,
    ) -> json::GetBlockchainInfoResult {
        let rpc_client = bitcoincore_rpc::Client::new(rpc_addr, rpc_auth).unwrap();
        let out = rpc_client.get_blockchain_info().unwrap();
        tracing::info!("get_blockchain_info: {:?}", out);
        out
    }

    #[cached(time = 60)]
    pub fn get_block_info(
        rpc_addr: String,
        rpc_auth: bitcoincore_rpc::Auth,
        hash: bitcoin::BlockHash,
    ) -> json::GetBlockResult {
        let rpc_client = bitcoincore_rpc::Client::new(rpc_addr, rpc_auth).unwrap();
        let out = rpc_client.get_block_info(&hash).unwrap();
        tracing::info!("get_block_info: {:?}", out);
        out
    }

    /// this method is not in the library yet
    #[cached(time = 60)]
    pub fn get_block_stats(
        rpc_addr: String,
        rpc_auth: bitcoincore_rpc::Auth,
        hash: bitcoin::BlockHash,
    ) -> BlockStatsInfo {
        let agent: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .build();
        let auth_hdr = if let bitcoincore_rpc::Auth::UserPass(user, pass) = rpc_auth {
            let auth_token = format!("{}:{}", user, pass);
            format!("Basic {}", base64::encode(auth_token))
        } else {
            "".to_owned()
        };
        let payload = format!(
            "{{\"jsonrpc\":\"1.0\",\"id\":\"{}\",\"method\":\"getblockstats\",\"params\":[\"{}\"]}}",
            hash, hash,
        );
        let stats_str: String = agent
            .post(rpc_addr.as_str())
            .set("Authorization", auth_hdr.as_str())
            .set("Content-Type", "application/json")
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
}

pub async fn home(req: Request<State>) -> Result {
    let state = req.state();
    let chaininfo = rpc::get_blockchain_info(state.rpc_addr.clone(), state.rpc_auth.clone());
    let block = rpc::get_block_info(
        state.rpc_addr.clone(),
        state.rpc_auth.clone(),
        chaininfo.best_block_hash,
    );
    let _ = rpc::get_block_stats(
        state.rpc_addr.clone(),
        state.rpc_auth.clone(),
        chaininfo.best_block_hash,
    );

    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("app", "bitcoin-explorer".to_owned());
    m.insert("blocks", format!("{}", chaininfo.blocks));
    m.insert("difficulty", format!("{}", chaininfo.difficulty));
    if let Some(previousblockhash) = block.previousblockhash {
        m.insert("previous_block", format!("{:?}", previousblockhash));
    }

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&m)?);
    Ok(res)
}
