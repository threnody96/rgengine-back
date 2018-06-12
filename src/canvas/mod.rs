use std::cell::RefCell;
use ::sdl2::rect::{Rect, Point};
use ::sdl2::render::{ Canvas, Texture, RenderTarget, TextureCreator };
use std::cmp::{max, min};
use std::rc::Rc;

#[derive(Clone)]
pub struct VirtualCanvas<'l, W, T> where W: RenderTarget + 'l, T: 'l {
    canvas: &'l RefCell<Canvas<W>>,
    texture_creator: &'l TextureCreator<T>,
    texture: Option<Rc<Texture<'l>>>,
    rect: Option<Rect>
}

impl<'l, W, T> VirtualCanvas<'l, W, T> where W: RenderTarget + 'l, T: 'l {

    pub fn new(canvas: &'l RefCell<Canvas<W>>, rect: Rect, texture_creator: &'l TextureCreator<T>) -> Self {
        Self {
            canvas: canvas,
            texture: Some(Rc::new(texture_creator.create_texture_static(None, rect.width(), rect.height()).unwrap())),
            texture_creator: texture_creator,
            rect: Some(rect)
        }
    }

    pub fn sub_canvas(&self, rect: Rect, f: &'l Fn(VirtualCanvas<'l, W, T>)) {
        if self.rect.is_none() { 
            f(Self { rect: None, texture: None, .. *self });
        } else {
            let cr = self.rect.unwrap();
            let nr = Rect::new(cr.x() + rect.x(), cr.y() + rect.y(), rect.width(), rect.height());
            f(Self { rect: Self::overlap_rect(&cr, &nr), texture: Some(Rc::new(self.texture_creator.create_texture_static(None, 10, 10).unwrap())), .. *self });
        }
    }

    pub fn tcopy(&self, t: &Texture, p: Point, clip: Option<Rect>) -> Result<(), String> {
        if self.rect.is_none() { return Ok(()); }
        let tq = t.query();
        let canvas_rect = self.rect.unwrap();
        let real_clip = clip.unwrap_or(Rect::new(0, 0, tq.width, tq.height));
        let real_point = Point::new(p.x() + canvas_rect.x() - (real_clip.width() as i32) / 2, p.y() + canvas_rect.y() - (real_clip.height() as i32) / 2);
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
