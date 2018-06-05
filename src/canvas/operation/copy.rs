extern crate sdl2;

use super::operation::Operation;
use self::sdl2::render::{Texture, BlendMode, Canvas};
use self::sdl2::rect::{Rect};
use self::sdl2::video::Window;
use super::super::VirtualCanvas;

pub struct CopyOperation<'a> {
    vcanvas: VirtualCanvas,
    texture: &'a Texture<'a>,
    src: Option<Rect>,
    dst: Option<Rect>
}

impl<'a> CopyOperation<'a> {

    pub fn new(vcanvas: &VirtualCanvas, texture: &'a Texture<'a>, src: Option<Rect>, dst: Option<Rect>) -> Self {
        Self { vcanvas: vcanvas.clone(), texture: texture, src: src, dst: dst }
    }

}

impl<'a> Operation for CopyOperation<'a> {

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        match self.vcanvas.rect {
            None => { Ok(()) },
            Some(rect) => {
                let vp = canvas.viewport();
                canvas.set_viewport(rect.clone());
                let r = canvas.copy(self.texture, self.src, self.dst);
                canvas.set_viewport(vp);
                r
            }
        }
    }

}
