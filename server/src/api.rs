use crate::rpc;
use crate::State;
use bitcoin::hashes::hex::FromHex;
// use chrono::prelude::*;
// use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::str::FromStr;
use std::collections::BTreeMap;
use tide::{Body, Request, Response, Result};

use bitcoincore_rpc_json as json;
use json::bitcoin;

pub async fn home(req: Request<State>) -> Result {
    let rpcclient = req.state().rpc_client.clone();
    let chaininfo = rpc::get_blockchain_info(rpcclient.clone());
    let mut m: BTreeMap<&str, String> = BTreeMap::new();
    m.insert("app", "bitcoin-explorer".to_owned());
    m.insert("blocks", format!("{}", chaininfo.blocks));
    m.insert("difficulty", format!("{}", chaininfo.difficulty));
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&m)?);
    Ok(res)
}

fn invalid_param(str: String) -> tide::Result {
    let mut m: BTreeMap<&'static str, String> = BTreeMap::new();
    m.insert("error", str.clone());
    let mut res = Response::new(400);
    res.set_body(tide::Body::from_json(&m)?);
    Ok(res)
}

pub async fn transaction(req: Request<State>) -> Result {
    let tx_id = match req.param("tx") {
        Ok(x) => x,
        Err(e) => return invalid_param(format!("missing tx param {}", e)),
    };
    let tx = match bitcoin::Txid::from_hex(tx_id) {
        Ok(x) => x,
        Err(e) => return invalid_param(format!("tx param parsing error {}", e)),
    };
    let rpcclient = req.state().rpc_client.clone();
    let rpcresult = rpc::get_raw_transaction_info(rpcclient.clone(), tx);
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&rpcresult)?);
    Ok(res)
}

pub async fn block(req: Request<State>) -> Result {
    let block_hash = match req.param("block") {
        Ok(x) => x,
        Err(e) => return invalid_param(format!("missing block param {}", e)),
    };
    let block = match bitcoin::BlockHash::from_hex(block_hash) {
        Ok(x) => x,
        Err(e) => return invalid_param(format!("tx param parsing error {}", e)),
    };
    let rpcclient = req.state().rpc_client.clone();
    let pg = crate::pager::Input::from_request(req);
    let rpcresult = rpc::get_block_info(rpcclient.clone(), block, pg);
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&rpcresult)?);
    Ok(res)
}

pub async fn blocks(req: Request<State>) -> Result {
    let rpcclient = req.state().rpc_client.clone();
    let pg = crate::pager::Input::from_request(req);
    let rpcresult = rpc::get_latest_blocks(rpcclient.clone(), pg);
    let mut res = Response::new(if rpcresult.is_invalid() { 400 } else { 200 });
    res.set_body(Body::from_json(&rpcresult)?);
    Ok(res)
}

pub async fn address(req: Request<State>) -> Result {
    let address_str = match req.param("address") {
        Ok(x) => x,
        Err(e) => return invalid_param(format!("missing address param {}", e)),
    };
    let address = match bitcoin::Address::from_str(address_str) {
        Ok(x) => x,
        Err(e) => return invalid_param(format!("address param error {}", e)),
    };
    let rpcclient = req.state().rpc_client.clone();
    let pg = crate::pager::Input::from_request(req);
    let rpcresult = rpc::get_address_history(rpcclient.clone(), address, pg);
    let mut res = Response::new(if rpcresult.is_invalid() { 400 } else { 200 });
    res.set_body(Body::from_json(&rpcresult)?);
    Ok(res)
}

pub async fn search(req: Request<State>) -> Result {
    let _rpcclient = req.state().rpc_client.clone();
    // let chaininfo = rpc::get(rpcclient.clone());
    // TODO: address, string, block
    let mut m: BTreeMap<&str, String> = BTreeMap::new();
    m.insert("endpoint", "search".to_owned());
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&m)?);
    Ok(res)
}
