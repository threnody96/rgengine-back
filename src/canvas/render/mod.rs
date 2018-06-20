use std::rc::Rc;
use std::cell::{ Ref, RefMut, RefCell };
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use ::sdl2::video::{ Window, WindowContext };
use super::VirtualCanvas;

pub struct TextureRenderer {
    canvas: Rc<RefCell<Canvas<Window>>>,
    texture: RefCell<Texture>
}

impl TextureRenderer {

    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, texture: RefCell<Texture>) -> Self {
        Self { canvas: canvas, texture: texture }
    }

    pub fn create(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>, width: u32, height: u32) -> Self {
        let o = texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, width, height).unwrap();
        TextureRenderer::new(canvas.clone(), RefCell::new(o))
    }

    pub fn clone(&self, texture_creator: Rc<TextureCreator<WindowContext>>) -> Self {
        let o = Self::create(self.canvas.clone(), texture_creator, self.width(), self.height());
        o.clear(VirtualCanvas::default_color())
            .copy(&self.borrow(), self.center(), None, 0.0);
        o
    }

    pub fn borrow(&self) -> Ref<Texture> { self.texture.borrow() }

    pub fn borrow_mut(&self) -> RefMut<Texture> { self.texture.borrow_mut() }

    pub fn width(&self) -> u32 { self.borrow().query().width }

    pub fn height(&self) -> u32 { self.borrow().query().height }

    pub fn center(&self) -> Point { Point::new(self.width() as i32 / 2, self.height() as i32 / 2) }

    pub fn fill_rect(&self, color: Color, rect: Rect) -> &Self {
        self.vcanvas_render(|c| {
            c.set_draw_color(color);
            c.fill_rect(rect);
        })
    }

    pub fn set_blend_mode(&self, mode: BlendMode) -> &Self {
        self.borrow_mut().set_blend_mode(mode);
        self
    }

    pub fn clear(&self, color: Color) -> &Self {
        self.vcanvas_render(|c| {
            c.set_draw_color(color);
            c.clear();
        })
    }

    pub fn copy(&self, t: &Texture, p: Point, clip: Option<Rect>, angle: f64) -> &Self {
        let draw_rect = self.get_draw_rect(&t, p, clip);
        self.vcanvas_copy(&t, clip, draw_rect, angle)
    }

    pub fn zoom(&self, t: &Texture, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64) -> &Self {
        let tmp_draw_rect = self.get_draw_rect(&t, p, clip);
        let draw_rect = Rect::new(
            tmp_draw_rect.x(),
            tmp_draw_rect.y(),
            ((tmp_draw_rect.width() as f32) * zoom_x.unwrap_or(1.0)) as u32,
            ((tmp_draw_rect.height() as f32) * zoom_y.unwrap_or(1.0)) as u32,
        );
        self.vcanvas_copy(&t, clip, draw_rect, angle)
    }

    fn vcanvas_copy(&self, t: &Texture, src: Option<Rect>, dst: Rect, angle: f64) -> &Self {
        self.vcanvas_render(|c| {
            c.copy_ex(&t, src, Self::convert_to_center_base_dst(dst, angle), angle, None, false, false).unwrap();
        })
    }

    pub fn vcanvas_render<F>(&self, f: F) -> &Self where for<'r> F: FnOnce(&'r mut Canvas<Window>,) {
        self.canvas.borrow_mut().with_texture_canvas(&mut self.borrow_mut(), f).map_err(|_| "sub canvas render error".to_owned()).unwrap();
        self
    }

    fn convert_to_center_base_dst(dst: Rect, angle: f64) -> Rect {
        if angle == 0.0 { return dst; }
        let bc = VirtualCanvas::calc_bounding_rect(dst, angle).center();
        let dc = dst.center();
        Rect::new(dst.x() + (dc.x() - bc.x()), dst.y() + (dc.y() - bc.y()), dst.width(), dst.height())
    }

    fn get_draw_rect(&self, t: &Texture, p: Point, clip: Option<Rect>) -> Rect {
        let tq = t.query();
        match clip {
            None => { Rect::from_center(p, tq.width, tq.height) },
            Some(cl) => { Rect::from_center(p, cl.width(), cl.height()) }
        }
    }

}
