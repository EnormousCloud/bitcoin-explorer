use crate::rpc;
use crate::State;
// use chrono::prelude::*;
// use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::collections::HashMap;
use tide::{Body, Request, Response, Result};

pub async fn home(req: Request<State>) -> Result {
    let rpcclient = req.state().rpc_client.clone();
    let chaininfo = rpc::get_blockchain_info(rpcclient.clone());
    let block = rpc::get_block_info(rpcclient.clone(), chaininfo.best_block_hash);
    let _ = rpc::get_block_stats(rpcclient.clone(), chaininfo.best_block_hash);

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
