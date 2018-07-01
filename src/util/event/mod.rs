use ::sdl2::EventPump;
use self::types::KeyType;

pub trait InputDataCore {

}

pub struct InputData {
    key: KeyType,
    state: bool,
    frame: u32,
    core: Box<InputDataCore>
}

pub mod types;
