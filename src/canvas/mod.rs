extern crate sdl2;

use canvas::sdl2::rect::{Point, Rect};
use canvas::sdl2::render::{Canvas, Texture, RenderTarget};
use std::cmp::{max, min};

pub struct VirtualCanvas<T: RenderTarget> {
    rect: Rect,
    canvas: Canvas<T>,
}

impl<T: RenderTarget> VirtualCanvas<T> {

    pub fn new(rect: Rect, canvas: Canvas<T>) -> Self {
        Self { rect: rect, canvas: canvas }
    }

    pub fn render_texture(&mut self, texture: &Texture, point: &Point, clip: &Option<Rect>) {
        let q = texture.query();
        let render_area = match clip {
            None => { Rect::new(0, 0, q.width.clone(), q.height.clone()) },
            Some(c) => {
                c.clone()
            }
        };
        match Self::overlap_rect(&self.rect, &render_area) {
            None => {},
            Some(r) => {
                self.canvas.copy(
                    &texture,
                    Rect::new(
                        render_area.x().clone(),
                        render_area.y().clone(),
                        r.width().clone(),
                        r.height().clone()
                    ),
                    Rect::new(
                        point.x().clone(), 
                        point.y().clone(), 
                        r.width().clone(), 
                        r.height().clone()
                    )
                ).unwrap();
            }
        };
    }

    fn overlap_rect(rect1: &Rect, rect2: &Rect) -> Option<Rect> {
        let l_x = max(rect1.x(), rect2.x());
        let r_x = min(rect1.x() + (rect1.width() as i32), rect2.x());
        let u_y = max(rect1.y(), rect2.y());
        let b_y = min(rect1.y() + (rect1.height() as i32), rect2.y() + (rect2.height() as i32));
        if r_x - l_x < 0 || b_y - u_y < 0 { return None };
        Some(Rect::new(l_x, u_y, (r_x - l_x) as u32, (b_y - u_y) as u32))
    }

}
