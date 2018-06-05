extern crate sdl2;
use super::operation::Operation;
use self::sdl2::render::Canvas;
use self::sdl2::video::Window;

pub struct OperationSet {
    operations: Vec<Box<Operation>>
}

impl OperationSet {

    pub fn new() -> Self {
        Self { operations: vec![] }
    }

    pub fn from(operation: Box<Operation>) -> Self {
        Self { operations: vec![operation] }
    }

    pub fn push(&mut self, operation: Box<Operation>) {
        self.operations.push(operation);
    }

}

impl Operation for OperationSet {

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for o in &self.operations {
            try!(o.render(canvas))
        }
        Ok(())
    }

}
