#[derive(Serialize)]
struct Req {
    param: String,
}

#[derive(Deserialize)]
struct Resp {
    ret_code: String,
    info: String,
}