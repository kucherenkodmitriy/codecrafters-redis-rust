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

    pub fn echo(message: impl Into<String>) -> Self {
        RespResponse::BulkString(Some(message.into().into_bytes()))
    }

    pub fn set() -> Self {
        RespResponse::SimpleString("OK".to_string())
    }

    pub fn get(value: impl Into<String>) -> Self {
        RespResponse::BulkString(Some(value.into().into_bytes()))
    }

    pub fn null() -> Self {
        RespResponse::BulkString(None)
    }

    pub fn to_resp(self) -> String {
        match self {
            RespResponse::SimpleString(s) => format!("+{}\r\n", s),
            RespResponse::Error(e) => format!("-{}\r\n", e),
            RespResponse::Integer(i) => format!(":{}\r\n", i),
            Self::BulkString(None) => "$-1\r\n".to_string(),
            Self::BulkString(Some(bytes)) => {
                format!("${}\r\n{}\r\n", bytes.len(), String::from_utf8_lossy(&bytes))
            },
            Self::Array(arr) => {
                let mut resp = format!("*{}\r\n", arr.len());
                for item in arr {
                    resp.push_str(&item.to_resp());
                }
                resp
            }
        }
    }
}