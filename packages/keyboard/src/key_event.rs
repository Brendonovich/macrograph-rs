use super::key::Key;

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: Key,
    pub pressed: bool,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
    pub meta_pressed: bool,
}
