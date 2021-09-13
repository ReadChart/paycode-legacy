extern crate reqwest;
extern crate serde;

use actix_web::web::Json;
use hex::{decode, encode};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::result_services::{
    cipher_default,
    decipher_default,
};

#[derive(Serialize)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Resp {
    ret_code: String,
    qr_code: String,
    qr_code_id: u64,
    exp_time: String,
    create_dtm: String,
    code_type: u8,
    code_acc_type: u8,
    acc_list: Vec<>,
    Time_in_millis:,
}

pub struct RequestData {
    status_code: u8,
    qrcode: String,
    msg: String,
}

const URL: &str = "https://pay.dgut.lerongsoft.com/dgutccl-web/v2/front/genQrCode";

pub fn resolve(ecard_id: String, acc_tr_type: u8) {
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
        .send();
    // TODO Missing handle and resolve resp data
    match res {
        Ok(res) => {
            let handled: EncodedResp = serde_json::from_str(&*res.text().unwrap()).unwrap();
            let decode_into_hex = decode(handled.info).unwrap();
            println!("{} {:?}", handled.ret_code, decipher_default(&decode_into_hex));
        }
        Err(_) => println!("Something Gone wrong"),
    }
}