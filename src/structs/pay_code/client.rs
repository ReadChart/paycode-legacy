extern crate serde;

use rbatis::crud::CRUDTable;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayCodeDto {
    pub card_id: Option<String>,
    pub account_type: Option<u8>,
}
impl_field_name_method!(PayCodeDto {
    card_id,
    account_type
});
impl Default for PayCodeDto {
    fn default() -> Self {
        PayCodeDto {
            card_id: None,
            account_type: None,
        }
    }
}

impl CRUDTable for PayCodeDto {
    fn table_name() -> String {
        "t_query_log".to_string()
    }

    fn table_columns() -> String {
        "card_id,account_type".to_string()
    }
}
