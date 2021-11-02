use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::key::Key;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyEvent {
  pub key: Key,
  pub shift_pressed: bool,
  pub ctrl_pressed: bool,
  pub alt_pressed: bool,
  pub meta_pressed: bool,
}

impl KeyEvent {
  pub fn to_value(&self) -> Value {
    serde_json::to_value(self).unwrap()
  }
}
