extern crate derive_more;
extern crate log;
extern crate reqwest;
extern crate serde;

use actix_web::Result;
use hex::{decode, encode};
use log::info;
use reqwest::blocking::Client;

use crate::errors::resolve_errors::ResolveError;
use crate::services::result_services::{
    cipher_default,
    decipher_default,
};
use crate::structs::pay_code::{
    client::PayCodeClientRespBody,
    server::{
        CipheredReq,
        CipheredResp,
        Req,
        Resp,
    },
};

const URL: &str = "https://pay.dgut.lerongsoft.com/dgutccl-web/v2/front/genQrCode";

pub fn resolve(ecard_id: String, acc_tr_type: u8) -> Result<PayCodeClientRespBody, ResolveError> {
    // Create client
    let client = Client::new();
    // Sending request
    let res = client
        .post(URL)
        .header("Content-Type", "application/json")
        .json(
            &CipheredReq::new(
                &encode(
                    cipher_default(
                        serde_json::to_string(
                            &Req::new(
                                String::from(&ecard_id),
                                0,
                                "genQrCode".to_string(),
                                acc_tr_type,
                                0)
                        ).unwrap()
                    )
                )
            )
        )
        .send()
        .map_err(|_e| ResolveError::UpstreamRespError)?;
    let res_handler = serde_json::from_str(&res.text().unwrap()).map_err(|_e| ResolveError::UpstreamRespUnreadable)?;
    let handled_res: CipheredResp = res_handler;
    match handled_res.info().is_none() {
        true => Err(ResolveError::IncorrectParam),
        _ => {
            let decoded = decode(handled_res.info().as_ref().unwrap()).map_err(|_e| ResolveError::DecodeIntoHexError)?;
            let deciphered = decipher_default(&decoded).map_err(|_e| ResolveError::DecipherError)?;
            let final_res = serde_json::from_str(&deciphered).map_err(|_e| ResolveError::DecodeIntoUTF8Error)?;
            let final_res_handled: Resp = final_res;
            info!("request success with ecarid: {} account type: {}", String::from(&ecard_id), acc_tr_type);
            Ok(PayCodeClientRespBody {
                status_code: 0,
                qrcode: final_res_handled.qr_code().clone(),
                msg: "QrCode Request Success".to_string(),
            })
        }
    }

}