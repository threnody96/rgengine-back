extern crate sdl2;

use std::cell::RefCell;
use self::sdl2::rect::{Rect, Point};
use self::sdl2::render::{ Canvas, Texture };
use self::sdl2::video::Window;
use std::cmp::{max, min};

#[derive(Clone)]
pub struct VirtualCanvas<'a> {
    canvas: &'a RefCell<Canvas<Window>>,
    rect: Option<Rect>
}

impl<'a> VirtualCanvas<'a> {

    pub fn new(canvas: &'a RefCell<Canvas<Window>>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        Self { canvas: canvas, rect: Some(Rect::new(0, 0, w, h)) }
    }

    pub fn sub_canvas(&self, rect: Rect) -> Self {
        if self.rect.is_none() { return Self { canvas: self.canvas, rect: None }; }
        let cr = self.rect.unwrap();
        let nr = Rect::new(cr.x() + rect.x(), cr.y() + rect.y(), rect.width(), rect.height());
        Self { canvas: self.canvas, rect: Self::overlap_rect(&cr, &nr) }
    }

    pub fn tcopy(&self, t: &Texture, p: Point, clip: Option<Rect>) -> Result<(), String> {
        if self.rect.is_none() { return Ok(()); }
        let tq = t.query();
        let canvas_rect = self.rect.unwrap();
        let real_clip = clip.unwrap_or(Rect::new(0, 0, tq.width, tq.height));
        let real_point = Point::new(p.x() + canvas_rect.x(), p.y() + canvas_rect.y());
        let mut c = self.canvas.borrow_mut();
        c.set_viewport(canvas_rect);
        c.copy(t, real_clip, Some(Rect::new(real_point.x(), real_point.y(), real_clip.width(), real_clip.height())))
    }

    fn overlap_rect(rect1: &Rect, rect2: &Rect) -> Option<Rect> {
        let l_x = max(rect1.x(), rect2.x());
        let r_x = min(rect1.x() + (rect1.width() as i32), rect2.x() + (rect2.width() as i32));
        let u_y = max(rect1.y(), rect2.y());
        let b_y = min(rect1.y() + (rect1.height() as i32), rect2.y() + (rect2.height() as i32));
        if r_x - l_x < 0 || b_y - u_y < 0 { return None };
        Some(Rect::new(l_x, u_y, (r_x - l_x) as u32, (b_y - u_y) as u32))
    }

}
