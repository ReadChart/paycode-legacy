#[derive(Serialize)]
struct Req {
    param: String,
}

#[derive(Deserialize)]
struct Resp {
    ret_code: String,
    info: String,
}

pub struct RequestData {
    status_code: u8,
    qrcode: String,
    msg: String,
}

pub fn resolve() -> RequestData {}