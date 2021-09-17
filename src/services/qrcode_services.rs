extern crate derive_more;
extern crate reqwest;
extern crate serde;

use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    HttpResponse,
    Result,
};
use derive_more::{Display, Error};
use hex::{decode, encode};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::errors::resolve_errors::ResolveError;
use crate::services::result_services::{
    cipher_default,
    decipher_default,
};

#[derive(Serialize, Debug)]
struct EncodedReq {
    param: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Req {
    ecard_id: String,
    code_type: u8,
    tr_type: String,
    acc_tr_type: u8,
    request_flag: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EncodedResp {
    ret_code: String,
    info: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StatusList {
    acc_tr_type: String,
    acc_amt: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    ret_code: String,
    qr_code: String,
    qr_code_id: u64,
    exp_time: String,
    create_dtm: String,
    code_type: String,
    code_acc_type: String,
    acc_list: Vec<StatusList>,
    distributed_key: String,
    prev_distributed_key: String,
    account_status: String,
    overdraft_amount: String,
    identityid: String,
    off_code_number: String,
}

#[derive(Deserialize, Serialize)]
pub struct RequestData {
    pub status_code: u8,
    pub qrcode: String,
    pub msg: String,
}

const URL: &str = "https://pay.dgut.lerongsoft.com/dgutccl-web/v2/front/genQrCode";

pub fn resolve(ecard_id: String, acc_tr_type: u8) -> Result<RequestData, ResolveError> {
    // Create client
    let client = Client::new();
    // Sending request
    let res = client
        .post(URL)
        .header("Content-Type", "application/json")
        .json(&EncodedReq {
            param: encode(cipher_default(serde_json::to_string(&Req {
                ecard_id,
                code_type: 0,
                tr_type: "genQrCode".to_string(),
                acc_tr_type,
                request_flag: 0,
            }).unwrap()))
        })
        .send()
        .map_err(|_e| ResolveError::UpstreamRespError)?;
    let res_handler = serde_json::from_str(&res.text().unwrap()).map_err(|_e| ResolveError::UpstreamRespUnreadable)?;
    let handled_res: EncodedResp = res_handler;
    let decoded = decode(handled_res.info).map_err(|_e| ResolveError::DecodeIntoHexError)?;
    let deciphered = decipher_default(&decoded).map_err(|_e| ResolveError::DecipherError)?;
    let final_res = serde_json::from_str(&deciphered).map_err(|_e| ResolveError::DecodeIntoUTF8Error)?;
    let final_res_handled: Resp = final_res;
    Ok(RequestData {
        status_code: 0,
        qrcode: final_res_handled.qr_code,
        msg: "QrCode Request Success".to_string(),
    })
}