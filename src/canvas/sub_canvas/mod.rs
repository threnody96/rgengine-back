use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use super::VirtualCanvas;

#[derive(Clone,Copy)]
pub struct VirtualCanvasOption {
    pub position: Rect,
    pub angle: f64,
    pub alpha: u8
}

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn sub_canvas(&self, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(option.position, option.alpha));
        self.do_sub_canvas(sub_canvas.clone(), option, f);
    }

    fn create_sub_canvas(&self, rect: Rect, alpha: u8) -> VirtualCanvas<'l, T> {
        let mut vcanvas = Self::create_new_vcanvas(&self.texture_creator, rect.width(), rect.height());
        vcanvas.set_alpha_mod(alpha);
        Self { vcanvas: RefCell::new(vcanvas), .. *self }
    }

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas<'l, T>>, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        let p = Point::new(option.position.x(), option.position.y());
        f(sub_canvas.clone());
        self.copy(&sub_canvas.vcanvas.borrow(), p, None, option.angle).unwrap();
    }

}
