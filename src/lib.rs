use std::collections::HashMap;

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

    pub fn set(&self, key: &'a str, value: &str) -> Option<(&'a str, String)> {
        let mut inner = self.inner.lock();
        inner
            .insert(key, value.to_string())
            .map(|value| (key, value))
    }
}

impl Default for Database<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Request<'a> {
    Get { key: &'a str },
    Set { key: &'a str, value: String },
}

impl<'a> Request<'a> {
    pub fn parse(input: &'a str) -> Result<Self, String> {
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("Invalid 'GET' request. Missing key")?;
                if parts.next().is_some() {
                    return Err(
                        "Invalid 'GET' request. Unexpected trailing argument.\nUsage: GET <key>"
                            .into(),
                    );
                }
                Ok(Request::Get { key })
            }
            Some("SET") => {
                let key = match parts.next() {
                    Some(k) => k,
                    None => return Err("Bad SET request.\nUsage: SET <key> <value>".into()),
                };
                let value = match parts.next() {
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
        previous: Option<&'a str>,
    },
    Error {
        message: &'a str,
    },
}
