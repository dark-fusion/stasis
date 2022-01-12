use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

pub const MAX_PAYLOAD_LENGTH: usize = 65536;

#[derive(Debug)]
pub struct Database {
    inner: Mutex<HashMap<String, String>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            inner: Mutex::default(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().get(key).cloned()
    }

    pub fn set(&self, key: &str, value: &str) -> Option<String> {
        self.inner.lock().insert(key.to_string(), value.to_string())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

pub fn handle_command(line: &str, database: &Arc<Database>) -> Response {
    let request = match Request::parse(line) {
        Ok(req) => req,
        Err(msg) => return Response::Error { message: msg },
    };

    match request {
        Request::Get { key } => match database.get(&key) {
            Some(value) => Response::Entry { key, value },
            None => Response::Error {
                message: format!("Missing key! {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = database.set(&key, &value);
            Response::Set {
                key,
                value,
                previous,
            }
        }
    }
}

pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
}

impl Request {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut tokens = input.splitn(3, ' ');
        match tokens.next() {
            Some("GET") => {
                let key = match tokens.next() {
                    Some(key) => key,
                    None => return Err("Bad GET request.\nUsage: GET <key>".into()),
                };

                if tokens.next().is_some() {
                    Err("Bad GET request. Unexpected argument.\nUsage: GET <key>".into())
                } else {
                    Ok(Request::Get {
                        key: key.to_string(),
                    })
                }
            }
            Some("SET") => {
                let key = match tokens.next() {
                    Some(k) => k,
                    None => return Err("Bad SET request.\nUsage: SET <key> <value>".into()),
                };

                let value = match tokens.next() {
                    Some(v) => v,
                    None => {
                        return Err("Bad SET request.\nUsage: SET <key> <value>".into());
                    }
                };

                Ok(Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(cmd) => Err(format!("Received unknown command: {}", cmd)),
            None => Err("Received empty input".into()),
        }
    }
}

pub enum Response {
    Entry {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        previous: Option<String>,
    },
    Error {
        message: String,
    },
}

impl Response {
    pub fn to_bytes(&self) -> String {
        match self {
            Response::Entry { key, value } => format!("{} => {}", key, value),
            Response::Set {
                key,
                value,
                previous,
            } => format!("{}: {}; previous: {:?}", key, value, previous),
            Response::Error { message } => format!("error: {}", message),
        }
    }
}
