#[derive(Debug, Clone, PartialEq)]
pub enum RespResponse {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    Array(Vec<RespResponse>),
}

impl RespResponse {
    pub fn pong() -> Self {
        RespResponse::SimpleString("PONG".to_string())
    }

    pub fn echo(message: impl Into<Vec<u8>>) -> Self {
        RespResponse::BulkString(Some(message.into()))
    }

    pub fn set() -> Self {
        RespResponse::SimpleString("OK".to_string())
    }

    pub fn get(value: impl Into<Vec<u8>>) -> Self {
        RespResponse::BulkString(Some(value.into()))
    }

    pub fn null() -> Self {
        RespResponse::BulkString(None)
    }

    pub fn to_resp(self) -> Vec<u8> {
        match self {
            RespResponse::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            RespResponse::Error(e) => format!("-{}\r\n", e).into_bytes(),
            RespResponse::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            Self::BulkString(None) => b"$-1\r\n".to_vec(),
            Self::BulkString(Some(bytes)) => {
                let mut resp = format!("${}\r\n", bytes.len()).into_bytes();
                resp.extend(bytes);
                resp.extend(b"\r\n");
                resp
            },
            Self::Array(arr) => {
                let mut resp = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    resp.extend(item.to_resp());
                }
                resp
            }
        }
    }
}