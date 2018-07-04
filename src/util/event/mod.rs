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
        Self { event: event, keyMap: RefCell::new(KeyMap::new()) }
    }

    pub fn update(&self) {
        let mut keymap = KeyMap::new();
        self.update_keys(&mut self.event.borrow_mut(), &mut keymap);
        self.keyMap.replace(keymap);
    }

    fn update_keys(&self, event: &mut EventPump, keymap: &mut KeyMap) {
        
    }

}

pub mod types;
pub mod values;
