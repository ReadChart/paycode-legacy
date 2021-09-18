extern crate serde;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayCodeClientReqBody {
    card_id: String,
    acc_type: u8,
}

impl PayCodeClientReqBody {
    pub fn get_card_id(&self) -> &String {
        &self.card_id
    }
    pub fn get_acc_type(&self) -> &u8 {
        &self.acc_type
    }
}

#[derive(Serialize)]
pub struct PayCodeClientRespBody {
    pub status_code: u8,
    pub qrcode: String,
    pub msg: String,
}