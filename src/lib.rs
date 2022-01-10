use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

pub mod channel;

#[derive(Debug)]
pub struct Database<'a> {
    inner: Mutex<HashMap<&'a str, String>>,
}

impl<'a> Database<'a> {
    pub fn new() -> Self {
        Self {
            inner: Mutex::default(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().get(key).cloned()
    }

    pub fn set(&self, key: &'a str, value: &str) -> Option<String> {
        self.inner.lock().insert(key, value.to_string())
    }
}

impl Default for Database<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn handle_command<'a>(line: &'a str, database: &Arc<Database<'a>>) -> Response<'a> {
    let request = match Request::parse(line) {
        Ok(req) => req,
        Err(msg) => return Response::Error { message: msg },
    };

    match request {
        Request::Get { key } => match database.get(key) {
            Some(value) => Response::Entry { key, value },
            None => Response::Error {
                message: format!("Missing key! {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = database.set(key, &value);
            Response::Set {
                key,
                value,
                previous,
            }
        }
    }
}

pub enum Request<'a> {
    Get { key: &'a str },
    Set { key: &'a str, value: String },
}

impl<'a> Request<'a> {
    pub fn parse(input: &'a str) -> Result<Self, String> {
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
                    Ok(Request::Get { key })
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
                    key,
                    value: value.to_string(),
                })
            }
            Some(cmd) => Err(format!("Received unknown command: {}", cmd)),
            None => Err("Received empty input".into()),
        }
    }
}

pub enum Response<'a> {
    Entry {
        key: &'a str,
        value: String,
    },
    Set {
        key: &'a str,
        value: String,
        previous: Option<String>,
    },
    Error {
        message: String,
    },
}

impl<'a> Response<'a> {
    pub fn to_body(&self) -> String {
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
