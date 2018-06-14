use std::cell::RefCell;
use ::sdl2::rect::Rect;
use self::operation::Operation;

pub struct ComponentOption {
    position: Rect,
    angle: f64,
    alpha: u8
}

pub struct Component<P, S> {
    option: ComponentOption,
    props: P,
    state: RefCell<S>,
    operations: RefCell<Vec<Box<Operation>>>
}

impl<P, S> Component<P, S> {
    
    pub fn new(option: ComponentOption, props: P, state: S) -> Self {
        Self { option: option, props: props, state: RefCell::new(state), operations: RefCell::new(Vec::new()) }
    }

    pub fn r(&self, operation: Box<Operation>) {
        self.operations.borrow_mut().push(operation);
    }

    pub fn render(&self) -> Vec<Box<Operation>> {
        let mut orig_operations = self.operations.borrow_mut();
        let mut operations: Vec<Box<Operation>> = Vec::new();
        while orig_operations.len() > 0 {
            operations.push(orig_operations.remove(0));
        }
        operations
    }

}

pub mod operation;
