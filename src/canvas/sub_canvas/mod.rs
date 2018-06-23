use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ BlendMode, Texture };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use super::{ VirtualCanvas, VirtualCanvasOption };
use ::util::texture::RGTexture;

impl VirtualCanvas {

    pub fn sub_canvas(&self, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas>)) {
        let sub_canvas = self.create_sub_canvas(option);
        f(sub_canvas.clone());
        self.copy_sub_canvas(sub_canvas);
    }

    fn create_new_sub_vcanvas(&self, br: Rect) -> Rc<RGTexture> {
        let vcanvas = Rc::new(RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), br.width(), br.height()));
        vcanvas.clean_copy(&*self.vcanvas, Some(br), Some(Rect::new(0, 0, br.width(), br.height())));
        vcanvas
    }

    fn create_sub_canvas(&self, option: VirtualCanvasOption) -> VirtualCanvas {
        let br = Self::calc_bounding_rect(option.position, option.angle);
        Self {
            canvas: self.canvas.clone(),
            vcanvas: self.create_new_sub_vcanvas(br),
            texture_creator: self.texture_creator.clone(),
            bounding_rect: br,
            option: option
        }
    }

    fn copy_sub_canvas_no_angle(&self, sub_canvas: Rc<VirtualCanvas>) {
        let t = sub_canvas.vcanvas.clone();
        t.set_blend_mode(BlendMode::Blend).set_texture_alpha(sub_canvas.option.alpha).emit();
        t.copy(t, sub_canvas.option.position.center(), None, sub_canvas.option.angle).emit();
    }

    fn copy_sub_canvas_with_angle(&self, sub_canvas: Rc<VirtualCanvas>) {
        let object = sub_canvas.create_transparent_object();
        let sc_texture = self.normalize_sub_canvas_texture(br, sub_canvas.clone(), object.clone());
    }

    fn copy_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>) {
        if sub_canvas.option.angle == 0.0 {
            self.copy_sub_canvas_no_angle(sub_canvas);
        } else {
            self.copy_sub_canvas_with_angle(sub_canvas);
        }
        let at_texture = self.create_sub_canvas_around_texture(br, sub_canvas.clone(), object.clone());
        sc_texture.copy(&at_texture.borrow(), sc_texture.center(), None, 0.0);
        self.vcanvas.fill_rect(Self::default_color(), br);
        self.vcanvas.copy(&sc_texture.borrow(), br.center(), None, 0.0);
    }

    fn create_sub_canvas_filter(&self, sub_canvas: Rc<VirtualCanvas>, object: Rc<RGTexture>) -> RGTexture {
        let br = self.bounding_rect;
        let t = RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), br.width(), br.height());
        t.copy(object.clone(), br.center(), None, sub_canvas.option.angle);
        t
    }

    fn normalize_sub_canvas_texture(&self, sub_canvas: Rc<VirtualCanvas>, object: Rc<RGTexture>) -> RGTexture {
        let t = RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), sub_canvas.option.width(), sub_canvas.option.height());
        t.clean_copy(&*self.vcanvas, Some(sub_canvas.bounding_rect), 
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

    fn create_transparent_object(&self) -> Rc<RGTexture> {
        let p = self.option.position;
        let object = RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), p.width(), p.height());
        object.set_blend_mode(BlendMode::None).clear(Some(Color::RGBA(0, 0, 0, 0))).emit();
        Rc::new(object)
    }

    fn create_renderer(&self, width: u32, height: u32) -> TextureRenderer {
        let o = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, width, height).unwrap();
        TextureRenderer::new(self.canvas.clone(), RefCell::new(o))
    }

}

pub mod calc;
