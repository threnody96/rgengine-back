use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use super::VirtualCanvas;

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn sub_canvas(&self, rect: Rect, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(rect));
        self.do_sub_canvas(sub_canvas.clone(), Point::new(rect.x(), rect.y()), f);
    }

    fn create_sub_canvas(&self, rect: Rect) -> VirtualCanvas<'l, T> {
        let mut vcanvas = self.texture_creator.create_texture_target(None, rect.width(), rect.height()).unwrap();
        self.canvas.borrow_mut().with_texture_canvas(&mut vcanvas, |c| {
            c.copy(&self.vcanvas.borrow(), Some(rect), Some(Rect::new(0, 0, rect.width(), rect.height()))).unwrap();
        }).unwrap();
        Self { vcanvas: RefCell::new(vcanvas), .. *self }
    }

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas<'l, T>>, p: Point, f: &Fn(Rc<VirtualCanvas<'l, T>>)) {
        f(sub_canvas.clone());
        self.copy(&sub_canvas.vcanvas.borrow(), p, None).unwrap();
    }

}
