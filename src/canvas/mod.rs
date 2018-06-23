use std::cell::RefCell;
use std::rc::Rc;
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use ::sdl2::rect::Rect;
use ::sdl2::video::{ Window, WindowContext };
use ::util::texture::RGTexture;

#[derive(Clone,Copy)]
pub struct VirtualCanvasOption {
    pub mode: BlendMode,
    pub position: Rect,
    pub angle: f64,
    pub alpha: u8
}

pub struct VirtualCanvas {
    canvas: Rc<RefCell<Canvas<Window>>>,
    vcanvas: Rc<RGTexture>,
    texture_creator: Rc<TextureCreator<WindowContext>>,
    bounding_rect: Rect,
    option: VirtualCanvasOption
}

impl VirtualCanvas {

    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        let option = VirtualCanvasOption { mode: BlendMode::None, position: Rect::new(0, 0, w, h), angle: 0.0, alpha: 255 };
        Self::initialize_canvas(canvas.clone());
        Self {
            canvas: canvas.clone(),
            vcanvas: Self::create_new_vcanvas(canvas.clone(), texture_creator.clone(), option),
            texture_creator: texture_creator,
            bounding_rect: option.position,
            option: option
        }
    }

    fn initialize_canvas(canvas: Rc<RefCell<Canvas<Window>>>) {
        let mut c = canvas.borrow_mut();
        c.set_blend_mode(BlendMode::None);
    }

    fn create_new_vcanvas(canvas: Rc<RefCell<Canvas<Window>>>, tc: Rc<TextureCreator<WindowContext>>, option: VirtualCanvasOption) -> Rc<RGTexture> {
        let p = option.position;
        let t = RGTexture::create(canvas, tc, p.width(), p.height());
        Rc::new(t)
    }

    pub fn render_to_real_canvas(&self) {
        self.canvas.borrow_mut().copy(
            &self.vcanvas.borrow(),
            None,
            Rect::new(0, 0, self.vcanvas.width(), self.vcanvas.height())
        ).unwrap();
        self.canvas.borrow_mut().present();
    }

}

pub mod sub_canvas;
pub mod operation_executer;
