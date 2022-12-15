use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct KVStoreError {
    message: String,
}

impl KVStoreError {
    pub fn new(message: &str) -> Self {
        KVStoreError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for KVStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for KVStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}