use core::fmt;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub struct BaseLogger {}

pub trait Logger: fmt::Debug + Serialize {
    fn get_log(&self) -> &str;
    fn log(&mut self, text: String);
}
