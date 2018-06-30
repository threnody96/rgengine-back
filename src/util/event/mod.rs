use ::sdl2::EventPump;
use self::types::KeyType;

pub struct InputData {
    key: KeyType,
    state: bool,
    frame: u32
}

pub mod types;
