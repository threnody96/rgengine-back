use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{max, min};
use ::sdl2::rect::{ Point, Rect };
use super::VirtualCanvas;

#[derive(Clone,Copy)]
pub struct VirtualCanvasOption {
    pub position: Rect,
    pub angle: f64,
    pub alpha: u8
}

impl VirtualCanvas {

    pub fn sub_canvas(&self, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(self.normalize_canvas_rect(option.position), option.alpha));
        self.do_sub_canvas(sub_canvas.clone(), option, f);
    }

    fn normalize_canvas_rect(&self, rect: Rect) -> Rect {
        let tq = self.vcanvas.borrow().query();
        let vcanvas_rect = Rect::new(0, 0, tq.width, tq.height);
        Self::overlap_rect(&rect, &vcanvas_rect).unwrap_or(Rect::new(0, 0, 1, 1))
    }

	fn overlap_rect(rect1: &Rect, rect2: &Rect) -> Option<Rect> {
        let l_x = max(rect1.x(), rect2.x());
        let r_x = min(rect1.x() + (rect1.width() as i32), rect2.x() + (rect2.width() as i32));
        let u_y = max(rect1.y(), rect2.y());
        let b_y = min(rect1.y() + (rect1.height() as i32), rect2.y() + (rect2.height() as i32));
        if r_x - l_x < 0 || b_y - u_y < 0 { return None };
        Some(Rect::new(l_x, u_y, (r_x - l_x) as u32, (b_y - u_y) as u32))
	}

    fn create_sub_canvas(&self, rect: Rect, alpha: u8) -> VirtualCanvas {
        let mut vcanvas = Self::create_new_vcanvas(self.texture_creator.clone(), rect.width(), rect.height());
        vcanvas.set_alpha_mod(alpha);
        Self {
            vcanvas: RefCell::new(vcanvas),
            canvas: self.canvas.clone(),
            texture_creator: self.texture_creator.clone()
        }
    }

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas>)) {
        let p = Point::new(option.position.x(), option.position.y());
        f(sub_canvas.clone());
        self.copy(&sub_canvas.vcanvas.borrow(), p, None, option.angle).unwrap();
    }

}
