use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::Texture;
use super::VirtualCanvas;

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn render(&self) {
        let tq = self.vcanvas.borrow().query();
        self.canvas.borrow_mut().copy(
            &self.vcanvas.borrow(),
            None,
            Rect::new(0, 0, tq.width, tq.height)
        ).unwrap();
        self.canvas.borrow_mut().present();
    }

    pub fn copy(&self, t: &Texture<'l>, p: Point, clip: Option<Rect>) -> Result<(), String> {
        let draw_rect = self.get_draw_rect(&t, p, clip);
        self.sub_canvas_copy(&t, clip, draw_rect)
    }

    pub fn zoom(&self, t: &Texture<'l>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>) -> Result<(), String> {
        let tmp_draw_rect = self.get_draw_rect(&t, p, clip);
        let draw_rect = Rect::new(
            tmp_draw_rect.x(),
            tmp_draw_rect.y(),
            ((tmp_draw_rect.width() as f32) * zoom_x.unwrap_or(1.0)) as u32,
            ((tmp_draw_rect.height() as f32) * zoom_y.unwrap_or(1.0)) as u32,
        );
        self.sub_canvas_copy(&t, clip, draw_rect)
    }

    fn sub_canvas_copy(&self, t: &Texture<'l>, src: Option<Rect>, dst: Rect) -> Result<(), String> {
        self.canvas.borrow_mut().with_texture_canvas(&mut self.vcanvas.borrow_mut(), |c| {
            c.copy(&t, src, Some(dst)).unwrap();
        }).map_err(|_| "sub canvas render error".to_owned())
    }

    fn get_draw_rect(&self, t: &Texture<'l>, p: Point, clip: Option<Rect>) -> Rect {
        let tq = t.query();
        match clip {
            None => { Rect::new(p.x(), p.y(), tq.width, tq.height) },
            Some(cl) => { Rect::new(p.x(), p.y(), cl.width(), cl.height()) }
        }
    }

}