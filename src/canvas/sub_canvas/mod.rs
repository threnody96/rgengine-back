use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ BlendMode, Texture };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use super::{ VirtualCanvas, VirtualCanvasOption };
use ::util::texture::RGTexture;

impl VirtualCanvas {

    pub fn sub_canvas(&self, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(option));
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
        self.vcanvas.copy(t, sub_canvas.option.position.center(), None, sub_canvas.option.angle).emit();
    }

    fn copy_sub_canvas_with_angle(&self, sub_canvas: Rc<VirtualCanvas>) {
        let object = sub_canvas.create_transparent_object();
        let sc_texture = self.normalize_sub_canvas_texture(sub_canvas.clone(), object.clone());
        let filter = self.create_sub_canvas_filter(sub_canvas.clone(), object.clone());
        sc_texture.copy_plain(&filter, None, None);
        self.vcanvas.clean_copy(&sc_texture, None, Some(sub_canvas.bounding_rect));
    }

    fn copy_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>) {
        if sub_canvas.option.angle == 0.0 {
            self.copy_sub_canvas_no_angle(sub_canvas);
        } else {
            self.copy_sub_canvas_with_angle(sub_canvas);
        }
    }

    fn create_sub_canvas_filter(&self, sub_canvas: Rc<VirtualCanvas>, object: Rc<RGTexture>) -> RGTexture {
        let br = sub_canvas.bounding_rect;
        let t = RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), br.width(), br.height());
        t.copy(object.clone(), br.center(), None, sub_canvas.option.angle).set_blend_mode(BlendMode::Blend).emit();
        t
    }

    fn normalize_sub_canvas_texture(&self, sub_canvas: Rc<VirtualCanvas>, object: Rc<RGTexture>) -> RGTexture {
        let br = sub_canvas.bounding_rect;
        let t = RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), br.width(), br.height());
        t.clean_copy(&*self.vcanvas, Some(sub_canvas.bounding_rect), Some(Rect::new(0, 0, br.width(), br.height())));
        t.copy(object.clone(), br.center(), None, sub_canvas.option.angle).set_blend_mode(BlendMode::Blend).emit();
        t
    }

    fn create_transparent_object(&self) -> Rc<RGTexture> {
        let p = self.option.position;
        let object = RGTexture::create(self.canvas.clone(), self.texture_creator.clone(), p.width(), p.height());
        object.set_blend_mode(BlendMode::None).clear(Some(Color::RGBA(0, 0, 0, 0))).emit();
        Rc::new(object)
    }

}

pub mod calc;
