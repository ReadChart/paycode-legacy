extern crate actix_web;
extern crate derive_more;
extern crate serde;

use std::fmt::{Display, Formatter};

use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{
        header,
        StatusCode,
    },
    HttpResponse,
};
use derive_more::{
    Display,
    Error,
};
use serde::Serialize;

#[derive(Debug, Serialize, Display, Error)]
pub enum ResolveError {
    #[display(fmt = "Failed To Decode Result Into Readable Result")]
    DecodeIntoUTF8Error,
    #[display(fmt = "Failed To Decipher Result")]
    DecipherError,
    #[display(fmt = "Unable To Request From Upstream")]
    UpstreamRespError,
    #[display(fmt = "Failed To Read Upstream Response")]
    UpstreamRespUnreadable,
    #[display(fmt = "Failed To Decode Message Into Hex")]
    DecodeIntoHexError,

}

#[derive(Serialize)]
pub struct ErrBody {
    status: u16,
    msg: String,
}

impl error::ResponseError for ResolveError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ResolveError::DecodeIntoUTF8Error => StatusCode::INTERNAL_SERVER_ERROR,
            ResolveError::DecipherError => StatusCode::INTERNAL_SERVER_ERROR,
            ResolveError::UpstreamRespError => StatusCode::BAD_REQUEST,
            ResolveError::UpstreamRespUnreadable => StatusCode::BAD_REQUEST,
            ResolveError::DecodeIntoHexError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(ErrBody { status: self.status_code().as_u16(), msg: self.to_string() })
    }
}