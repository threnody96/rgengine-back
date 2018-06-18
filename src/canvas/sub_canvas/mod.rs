use std::rc::Rc;
use std::cell::RefCell;
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ BlendMode, Texture };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use super::{ VirtualCanvas, VirtualCanvasOption };

impl VirtualCanvas {

    pub fn sub_canvas(&self, option: VirtualCanvasOption, f: &Fn(Rc<VirtualCanvas>)) {
        let sub_canvas = Rc::new(self.create_sub_canvas(option));
        self.do_sub_canvas(sub_canvas.clone(), f);
    }

    fn create_new_sub_vcanvas(&self, br: Rect) -> Texture {
        let mut vcanvas = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, br.width(), br.height()).unwrap();
        vcanvas.set_blend_mode(BlendMode::Blend);
        self.canvas.borrow_mut().with_texture_canvas(&mut vcanvas, |sc| {
            sc.copy(&self.vcanvas.borrow(), br, Rect::new(0, 0, br.width(), br.height())).unwrap();
        }).unwrap();
        vcanvas
    }

    fn create_sub_canvas(&self, option: VirtualCanvasOption) -> VirtualCanvas {
        let br = Self::calc_bounding_rect(option.position, option.angle);
        Self {
            canvas: self.canvas.clone(),
            vcanvas: RefCell::new(self.create_new_sub_vcanvas(br)),
            texture_creator: self.texture_creator.clone(),
            bounding_rect: br,
            option: option
        }
    }

    fn do_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>, f: &Fn(Rc<VirtualCanvas>)) {
        let p = Point::new(sub_canvas.option.position.x(), sub_canvas.option.position.y());
        f(sub_canvas.clone());
        self.copy(&sub_canvas.vcanvas.borrow(), p, None, sub_canvas.option.angle).unwrap();
    }

    fn copy_sub_canvas(&self, sub_canvas: Rc<VirtualCanvas>) {
        let o = Rc::new(sub_canvas.create_transparent_object());

    }

    fn create_sub_canvas_mask(&self, object: Rc<Texture>) -> Texture {
        let br = self.bounding_rect;
        let mut mask = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, br.width(), br.height()).unwrap();
        self.canvas.borrow_mut().with_texture_canvas(&mut mask, |sc| {
            sc.set_blend_mode(BlendMode::None);
            sc.set_draw_color(Self::default_color());
            sc.clear();
            sc.copy_ex(&object, None, self.option.position, self.option.angle, None, false, false).unwrap();
        }).unwrap();
        mask.set_blend_mode(BlendMode::Blend);
        mask
    }

    fn create_transparent_object(&self) -> Texture {
        let p = self.option.position;
        let mut o = self.texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, p.width(), p.height()).unwrap();
        self.canvas.borrow_mut().with_texture_canvas(&mut o, |sc| {
            sc.set_blend_mode(BlendMode::None);
            sc.set_draw_color(Color::RGBA(0, 0, 0, 0));
        }).unwrap();
        o
    }

}

pub mod calc;
