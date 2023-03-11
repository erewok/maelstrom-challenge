use serde::{Deserialize, Serialize};

// "Codes 0-999 are reserved for Maelstrom's use; codes 1000 and above are free for your own purposes."
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ErrorType {
    Timeout = 0,
    NodeNotFound = 1,
    NotSupported = 10,
    TemporarilyUnavailable = 11,
    MalformedRequest = 12,
    Crash = 13,
    Abort = 14,
    KeyDoesNotExist = 20,
    KeyAlreadyExists = 21,
    PreconditionFailed = 22,
    TxnConflict = 30,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ErrorMessageType {
    Error,
}

#[derive(Clone, Debug, Serialize)]
struct ErrorM(ErrorMessageType);

#[derive(Clone, Debug, Serialize)]
pub struct ErrorMsg {
    #[serde(rename = "type")]
    typ: ErrorM,
    pub in_reply_to: Option<u64>,
    pub code: ErrorType,
    pub text: String,
}

impl ErrorMsg {
    pub fn new(in_reply_to: Option<u64>, code: ErrorType, text: String) -> Self {
        ErrorMsg {
            typ: ErrorM(ErrorMessageType::Error),
            in_reply_to,
            code,
            text,
        }
    }

    pub fn json_parse_error() -> Self {
        ErrorMsg::new(
            None,
            ErrorType::MalformedRequest,
            "Request Parsing error".to_string(),
        )
    }

    pub fn json_dumps_error() -> Self {
        ErrorMsg::new(
            None,
            ErrorType::MalformedRequest,
            "Request Serialization error".to_string(),
        )
    }

    pub fn crash_error() -> Self {
        ErrorMsg::new(None, ErrorType::Crash, "Unrecoverable error".to_string())
    }
}
