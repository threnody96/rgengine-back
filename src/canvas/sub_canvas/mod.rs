use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ BlendMode, Texture };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use super::{ VirtualCanvas, VirtualCanvasOption };
use super::render::TextureRenderer;

impl VirtualCanvas {

    pub fn sub_canvas(&self, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(option));
        self.do_sub_canvas(sub_canvas, f);
    }

    fn create_new_sub_vcanvas(&self, br: Rect) -> TextureRenderer {
        let vcanvas = self.create_sub_canvas_base(br);
        vcanvas.set_blend_mode(BlendMode::Blend)
            .copy(&self.vcanvas.borrow(), br.center(), None, 0.0);
        vcanvas
    }

    fn create_sub_canvas(&self, option: VirtualCanvasOption) -> VirtualCanvas {
        Self {
            canvas: self.canvas.clone(),
            vcanvas: self.create_new_sub_vcanvas(Self::calc_bounding_rect(option.position, option.angle)),
            texture_creator: self.texture_creator.clone(),
            option: option
        }
    }

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>, f: &Fn(Rc<VirtualCanvas>)) {
        let p = Point::new(sub_canvas.option.position.x(), sub_canvas.option.position.y());
        f(sub_canvas.clone());
        self.copy_sub_canvas(sub_canvas);
    }

    fn copy_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>) {
        let br = Self::calc_bounding_rect(sub_canvas.option.position, sub_canvas.option.angle);
        let object = Rc::new(sub_canvas.create_transparent_object());
        let sc_texture = self.normalize_sub_canvas_texture(br, sub_canvas.clone(), object.clone());
        let at_texture = self.create_sub_canvas_around_texture(br, sub_canvas.clone(), object.clone());
        sc_texture.copy(&at_texture.borrow(), sc_texture.center(), None, 0.0);
        self.vcanvas.fill_rect(Self::default_color(), br);
        self.vcanvas.copy(&sc_texture.borrow(), br.center(), None, 0.0);
    }

    fn create_sub_canvas_around_texture(&self, br: Rect, sub_canvas: Rc<VirtualCanvas>, object: Rc<TextureRenderer>) -> TextureRenderer {
        let t = self.create_sub_canvas_base(br);
        t.set_blend_mode(BlendMode::None)
            .copy(&object.borrow(), sub_canvas.vcanvas.center(), None, sub_canvas.option.angle)
            .set_blend_mode(BlendMode::Blend);
        t
    }

    fn normalize_sub_canvas_texture(&self, br: Rect, sub_canvas: Rc<VirtualCanvas>, object: Rc<TextureRenderer>) -> TextureRenderer {
        let p = sub_canvas.vcanvas.center();
        let mask = self.create_sub_canvas_mask(sub_canvas.clone(), object.clone());
        let t = self.create_renderer(p.x() as u32, p.y() as u32);
        t.set_blend_mode(BlendMode::None)
            .copy(&sub_canvas.vcanvas.borrow(), p, None, 0.0)
            .set_blend_mode(BlendMode::Blend)
            .copy(&mask.borrow(), br.center(), None, 0.0);
        t
    }

    fn create_sub_canvas_base(&self, br: Rect) -> TextureRenderer {
        let t = self.create_renderer(br.width(), br.height());
        t.set_blend_mode(BlendMode::Blend)
            .clear(Self::default_color())
            .copy(&self.vcanvas.borrow(), br.center(), None, 0.0);
        t
    }

    fn create_sub_canvas_mask(&self, sub_canvas: Rc<VirtualCanvas>, object: Rc<TextureRenderer>) -> TextureRenderer {
        let mask = self.create_renderer(sub_canvas.vcanvas.width(), sub_canvas.vcanvas.height());
        mask.set_blend_mode(BlendMode::None)
            .clear(Self::default_color())
            .copy(&object.borrow(), sub_canvas.vcanvas.center(), None, sub_canvas.option.angle)
            .set_blend_mode(BlendMode::Blend);
        mask
    }

    fn create_transparent_object(&self) -> TextureRenderer {
        let p = self.option.position;
        let object = self.create_renderer(p.width(), p.height());
        object.set_blend_mode(BlendMode::None)
            .clear(Color::RGBA(0, 0, 0, 0));
        object
    }

    fn create_renderer(&self, width: u32, height: u32) -> TextureRenderer {
        let o = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, width, height).unwrap();
        TextureRenderer::new(self.canvas.clone(), RefCell::new(o))
    }

}

pub mod calc;
