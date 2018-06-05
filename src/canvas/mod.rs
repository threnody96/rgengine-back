extern crate sdl2;

use self::sdl2::rect::{Rect};
use self::sdl2::render::{ Canvas, Texture, TextureCreator };
use self::sdl2::surface::Surface;
use self::sdl2::video::Window;
use std::cmp::{max, min};

pub mod operation;

#[derive(Clone)]
pub struct VirtualCanvas {
    rect: Option<Rect>
}

impl VirtualCanvas {

    pub fn new(canvas: &Canvas<Window>) -> Self {
        let (w, h) = canvas.window().size();
        Self { rect: Some(Rect::new(0, 0, w, h)) }
    }

    pub fn sub_canvas(&self, rect: Rect, f: Box<Fn(Self) -> i32>) -> i32 {
        let sub_rect = match self.rect {
            None => { None },
            Some(current_rect) => {
                let s = Rect::new(
                    current_rect.x() + rect.x(),
                    current_rect.y() + rect.y(),
                    rect.width(),
                    rect.height()
                );
                Self::overlap_rect(&current_rect, &s)
            }
        };
        let sub_canvas = Self { rect: sub_rect };
        f(sub_canvas)
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
