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
        self.do_sub_canvas(sub_canvas.clone(), f);
    }

    fn create_new_sub_vcanvas(&self, br: Rect) -> TextureRenderer {
        let vcanvas = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, br.width(), br.height()).unwrap();
        let tr = TextureRenderer::new(self.canvas.clone(), RefCell::new(vcanvas));
        tr.set_blend_mode(BlendMode::Blend);
        tr.copy(&self.vcanvas.borrow(), br.center(), None, 0.0);
        tr
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

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>, f: &Fn(Rc<VirtualCanvas>)) {
        let p = Point::new(sub_canvas.option.position.x(), sub_canvas.option.position.y());
        f(sub_canvas.clone());
        self.copy_sub_canvas(sub_canvas);
    }

    fn copy_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>) {
        let object = Rc::new(sub_canvas.create_transparent_object());
    }

    fn create_transparent_object(&self) -> TextureRenderer {
        let p = self.option.position;
        let object = self.create_renderer(p.width(), p.height());
        object.set_blend_mode(BlendMode::None);
        object.clear(Color::RGBA(0, 0, 0, 0)).unwrap();
        object
    }

    fn create_renderer(&self, width: u32, height: u32) -> TextureRenderer {
        let o = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, width, height).unwrap();
        TextureRenderer::new(self.canvas.clone(), RefCell::new(o))
    }

}

pub mod calc;
