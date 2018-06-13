use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use super::VirtualCanvas;

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn sub_canvas(&self, rect: Rect, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        self.sub_alpha_canvas(rect, 255, f);
    }

    pub fn sub_alpha_canvas(&self, rect: Rect, alpha: u8, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(rect, alpha));
        self.do_sub_canvas(sub_canvas.clone(), Point::new(rect.x(), rect.y()), f);
    }

    fn create_sub_canvas(&self, rect: Rect, alpha: u8) -> VirtualCanvas<'l, T> {
        let mut vcanvas = Self::create_new_vcanvas(&self.texture_creator, rect.width(), rect.height());
        vcanvas.set_alpha_mod(alpha);
        Self { vcanvas: RefCell::new(vcanvas), .. *self }
    }

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas<'l, T>>, p: Point, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        f(sub_canvas.clone());
        self.copy(&sub_canvas.vcanvas.borrow(), p, None).unwrap();
    }

}
