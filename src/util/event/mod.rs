use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ::sdl2::EventPump;
use self::types::KeyType;
use self::values::KeyValue;

type KeyMap = HashMap<KeyType, KeyValue>;

pub struct InputData {
    event: Rc<RefCell<EventPump>>,
    keyMap: RefCell<KeyMap>
}

impl InputData {

    pub fn new(event: Rc<RefCell<EventPump>>) -> Self {
        Self { event: event, keyMap: RefCell::new(Self::initKeyMap()) }
    }

    fn initKeyMap() -> KeyMap {
        KeyMap::new()
    }

    pub fn update(&self) {
        let mut keymap = KeyMap::new();
        self.update_keys(&mut keymap);
        self.keyMap.replace(keymap);
    }

    fn update_keys(&self, keymap: &mut KeyMap) {
        let e = self.event.borrow_mut();
        
        let keyMap = self.keyMap.borrow();
    }

}

pub mod types;
pub mod values;
