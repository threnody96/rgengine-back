extern crate sdl2;

use super::operation::Operation;
use super::super::VirtualCanvas;
use self::sdl2::rect::Rect;
use self::sdl2::render::Canvas;
use self::sdl2::video::Window;

pub struct NoneOperation { }

impl NoneOperation {

    pub fn new() -> Self {
        Self {}
    }

}

impl Operation for NoneOperation {

    fn render(&self, _canvas: &mut Canvas<Window>) -> Result<(), String> {
        Ok(())
    }

}
