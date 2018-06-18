use std::cell::RefCell;
use std::rc::Rc;
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use ::sdl2::rect::Rect;
use ::sdl2::video::{ Window, WindowContext };
use self::render::TextureRenderer;

#[derive(Clone,Copy)]
pub struct VirtualCanvasOption {
    pub mode: BlendMode,
    pub position: Rect,
    pub angle: f64,
    pub alpha: u8
}

pub struct VirtualCanvas {
    canvas: Rc<RefCell<Canvas<Window>>>,
    vcanvas: TextureRenderer,
    texture_creator: Rc<TextureCreator<WindowContext>>,
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
            option: option
        }
    }

    fn initialize_canvas(canvas: Rc<RefCell<Canvas<Window>>>) {
        let mut c = canvas.borrow_mut();
        c.set_blend_mode(BlendMode::Blend);
    }

    fn create_new_vcanvas(canvas: Rc<RefCell<Canvas<Window>>>, tc: Rc<TextureCreator<WindowContext>>, option: VirtualCanvasOption) -> TextureRenderer {
        let p = option.position;
        let mut vcanvas = tc.create_texture_target(PixelFormatEnum::ARGB8888, p.width(), p.height()).unwrap();
        vcanvas.set_blend_mode(option.mode);
        canvas.borrow_mut().with_texture_canvas(&mut vcanvas, |sc| {
            sc.set_draw_color(Self::default_color());
            sc.clear();
        }).unwrap();
        TextureRenderer::new(canvas.clone(), RefCell::new(vcanvas))
    }

    pub fn render_to_real_canvas(&self) {
        self.canvas.borrow_mut().copy(
            &self.vcanvas.borrow(),
            None,
            Rect::new(0, 0, self.vcanvas.width(), self.vcanvas.height())
        ).unwrap();
        self.canvas.borrow_mut().present();
    }

    fn default_color() -> Color {
        Color::RGBA(0, 0, 0, 255)
    }

}

pub mod sub_canvas;
pub mod render;
pub mod operation_executer;
