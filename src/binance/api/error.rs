use std::fmt::Display;

use jsonrpc_v2::ErrorLike;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinanceErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl Display for BinanceErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("code: {} ", self.code)).unwrap();
        f.write_fmt(format_args!("error: {}", self.msg))
    }
}

type BoxedSerialize = Box<dyn erased_serde::Serialize + Send>;

impl ErrorLike for BinanceErrorResponse {
    fn code(&self) -> i64 {
        self.code
    }

    fn message(&self) -> String {
        self.msg.clone()
    }

    fn data(&self) -> Option<BoxedSerialize> {
        None
    }
}

// impl From<BinanceErrorResponse> for Error {
//     fn from(t: BinanceErrorResponse) -> Self {
//         Error::Provided { code: t.code, message: &t.msg }
//     }
// }
