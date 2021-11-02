
use rdev::{Key as RDevKey};

use super::key::Key;
fn match_key(key: RDevKey) -> Option<Key> {
  match key {
    RDevKey::KeyA => Some(Key::A),
    RDevKey::KeyB => Some(Key::B),
    RDevKey::KeyC => Some(Key::C),
    RDevKey::KeyD => Some(Key::D),
    RDevKey::KeyE => Some(Key::E),
    RDevKey::KeyF => Some(Key::F),
    RDevKey::KeyG => Some(Key::G),
    RDevKey::KeyH => Some(Key::H),
    RDevKey::KeyI => Some(Key::I),
    RDevKey::KeyJ => Some(Key::J),
    RDevKey::KeyK => Some(Key::K),
    RDevKey::KeyL => Some(Key::L),
    RDevKey::KeyM => Some(Key::M),
    RDevKey::KeyN => Some(Key::N),
    RDevKey::KeyO => Some(Key::O),
    RDevKey::KeyP => Some(Key::P),
    RDevKey::KeyQ => Some(Key::Q),
    RDevKey::KeyR => Some(Key::R),
    RDevKey::KeyS => Some(Key::S),
    RDevKey::KeyT => Some(Key::T),
    RDevKey::KeyU => Some(Key::U),
    RDevKey::KeyV => Some(Key::V),
    RDevKey::KeyW => Some(Key::W),
    RDevKey::KeyX => Some(Key::X),
    RDevKey::KeyY => Some(Key::Y),
    RDevKey::KeyZ => Some(Key::Z),
    _ => None,
  }
}
