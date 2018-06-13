use std::cell::RefCell;
use ::sdl2::rect::{Rect, Point};
use ::sdl2::render::{ Canvas, Texture, TextureCreator };
use ::sdl2::video::Window;

pub struct VirtualCanvas<'l, T> where T: 'l {
    canvas: &'l RefCell<Canvas<Window>>,
    vcanvas: RefCell<Texture<'l>>,
    texture_creator: &'l TextureCreator<T>,
}

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn new(canvas: &'l RefCell<Canvas<Window>>, texture_creator: &'l TextureCreator<T>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        Self {
            canvas: canvas,
            vcanvas: RefCell::new(texture_creator.create_texture_target(None, w, h).unwrap()),
            texture_creator: texture_creator
        }
    }

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
        let tq = t.query();
        let draw_rect = match clip {
            None => { Rect::new(p.x(), p.y(), tq.width, tq.height) },
            Some(cl) => { Rect::new(p.x(), p.y(), cl.width(), cl.height()) }
        };
        self.canvas.borrow_mut().with_texture_canvas(&mut self.vcanvas.borrow_mut(), |c| {
            c.copy(&t, clip, draw_rect).unwrap();
        }).map_err(|_| "sub canvas render error".to_owned())
    }

}

pub mod sub_canvas;
