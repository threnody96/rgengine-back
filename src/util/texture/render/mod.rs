use std::rc::Rc;
use super::RGTexture;
use super::operation::Operation;
use ::sdl2::pixels::Color;
use ::sdl2::render::Canvas;
use ::sdl2::video::Window;
use ::sdl2::rect::{ Point, Rect };

impl RGTexture {

    pub fn fill_rect(&self, color: Color, rect: Rect) -> &Self {
        self.regist(Operation::FillRect { color: color, rect: rect });
        self
    }

    pub fn do_fill_rect(&self, c: &mut Canvas<Window>, color: Color, rect: Rect) {
        let current_color = c.draw_color();
        c.set_draw_color(color);
        c.fill_rect(rect);
        c.set_draw_color(current_color);
    }

    pub fn clear(&self, color: Color) -> &Self {
        self.regist(Operation::Clear { color: color });
        self
    }

    pub fn do_clear(&self, c: &mut Canvas<Window>, color: Color) {
        let current_color = c.draw_color();
        c.set_draw_color(color);
        c.clear();
        c.set_draw_color(current_color);
    }

    pub fn copy(&self, t: Rc<RGTexture>, p: Point, clip: Option<Rect>, angle: f64) -> &Self {
        self.regist(Operation::Copy { t: t, p: p, clip: clip, angle: angle });
        self
    }

    pub fn do_copy(&self, c: &mut Canvas<Window>, t: Rc<RGTexture>, p: Point, clip: Option<Rect>, angle: f64) {
        let draw_rect = self.get_draw_rect(t.clone(), p, clip);
        c.copy_ex(&t.borrow(), clip, draw_rect, angle, None, false, false).unwrap();
    }

    pub fn zoom(&self, t: Rc<RGTexture>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64) -> &Self {
        self.regist(Operation::Zoom { t: t, p: p, clip: clip, zoom_x: zoom_x, zoom_y: zoom_y, angle: angle });
        self
    }

    pub fn do_zoom(&self, c: &mut Canvas<Window>, t: Rc<RGTexture>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64) {
        let tmp_draw_rect = self.get_draw_rect(t.clone(), p, clip);
        let draw_rect = Rect::new(
            tmp_draw_rect.x(),
            tmp_draw_rect.y(),
            ((tmp_draw_rect.width() as f32) * zoom_x.unwrap_or(1.0)) as u32,
            ((tmp_draw_rect.height() as f32) * zoom_y.unwrap_or(1.0)) as u32,
        );
        c.copy_ex(&t.borrow(), clip, draw_rect, angle, None, false, false).unwrap();
    }

    pub fn copy_plain(&self, t: Rc<RGTexture>, src: Option<Rect>, dst: Option<Rect>) -> &Self {
        self.regist(Operation::CopyPlain { t: t, src: src, dst: dst });
        self
    }

    pub fn do_copy_plain(&self, c: &mut Canvas<Window>, t: Rc<RGTexture>, src: Option<Rect>, dst: Option<Rect>) {
        c.copy(&t.borrow(), src, dst).unwrap();
    }

    fn get_draw_rect(&self, t: Rc<RGTexture>, p: Point, clip: Option<Rect>) -> Rect {
        let tq = t.borrow().query();
        match clip {
            None => { Rect::from_center(p, tq.width, tq.height) },
            Some(cl) => { Rect::from_center(p, cl.width(), cl.height()) }
        }
    }

}
