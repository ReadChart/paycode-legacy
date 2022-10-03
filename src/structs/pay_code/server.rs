extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CipheredReq {
    param: String,
}

impl CipheredReq {
    pub fn param(&self) -> &String {
        &self.param
    }
    pub fn new(param: &String) -> CipheredReq {
        CipheredReq {
            param: String::from(param),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Req {
    ecard_id: String,
    code_type: u8,
    tr_type: String,
    acc_tr_type: u8,
    request_flag: u8,
}

impl Req {
    pub fn ecard_id(&self) -> &String {
        &self.ecard_id
    }
    pub fn code_type(&self) -> &u8 {
        &self.code_type
    }
    pub fn tr_type(&self) -> &String {
        &self.tr_type
    }
    pub fn acc_tr_type(&self) -> &u8 {
        &self.acc_tr_type
    }
    pub fn request_flag(&self) -> &u8 {
        &self.request_flag
    }
    pub fn new(
        ecard_id: String,
        code_type: u8,
        tr_type: String,
        acc_tr_type: u8,
        request_flag: u8,
    ) -> Req {
        Req {
            ecard_id,
            code_type,
            tr_type,
            acc_tr_type,
            request_flag,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CipheredResp {
    ret_code: Option<String>,
    info: Option<String>,
}

impl CipheredResp {
    pub fn ret_code(&self) -> &Option<String> {
        &self.ret_code
    }
    pub fn info(&self) -> &Option<String> {
        &self.info
    }
    pub fn new(ret_code: Option<String>, info: Option<String>) -> CipheredResp {
        CipheredResp { ret_code, info }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusList {
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

impl Resp {
    pub fn qr_code(&self) -> &String {
        &self.qr_code
    }
}
