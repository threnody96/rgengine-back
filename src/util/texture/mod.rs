use std::rc::Rc;
use std::cell::{ Ref, RefMut, RefCell };
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use ::sdl2::video::{ Window, WindowContext };
use self::operation::Operation;
use self::log::OperationLog;

pub struct RGTexture {
    canvas: Rc<RefCell<Canvas<Window>>>,
    texture_creator: Rc<TextureCreator<WindowContext>>,
    texture: RefCell<Texture>,
    operations: RefCell<Vec<Operation>>,
    log: RefCell<OperationLog>
}

impl RGTexture {

    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>, texture: RefCell<Texture>) -> Self {
        let t = Self {
            canvas: canvas,
            texture_creator: texture_creator,
            texture: texture,
            operations: RefCell::new(Vec::new()),
            log: RefCell::new(OperationLog::new())
        };
        t.init();
        t
    }

    pub fn create(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>, width: u32, height: u32) -> Self {
        let o = texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, width, height).unwrap();
        let t = Self::new(canvas.clone(), texture_creator.clone(), RefCell::new(o));
        t.init();
        t
    }

    pub fn borrow(&self) -> Ref<Texture> { self.texture.borrow() }

    pub fn borrow_mut(&self) -> RefMut<Texture> { self.texture.borrow_mut() }

    pub fn width(&self) -> u32 { self.borrow().query().width }

    pub fn height(&self) -> u32 { self.borrow().query().height }

    fn regist(&self, operation: Operation) {
        self.operations.borrow_mut().push(operation);
    }

    pub fn emit(&self) -> &Self {
        if self.operations.borrow().len() == 0 { return self; }
        self.canvas.borrow_mut().with_texture_canvas(&mut self.borrow_mut(), |c| {
            while self.operations.borrow().len() > 0 {
                self.do_operation(c, self.operations.borrow_mut().remove(0));
            }
        }).unwrap();
        self
    }

    fn do_operation(&self, c: &mut Canvas<Window>, operation: Operation) {
         match operation {
             Operation::SetDrawColor { color } => { self.do_set_draw_color(c, color); },
             Operation::SetBlendMode { mode } => { self.do_set_blend_mode(c, mode); },
             Operation::FillRect { color, rect } => { self.do_fill_rect(c, color, rect); },
             Operation::Clear { color } => { self.do_clear(c, color); },
             Operation::Copy { t, p, clip, angle } => { self.do_copy(c, t, p, clip, angle); },
             Operation::Zoom { t, p, clip, zoom_x, zoom_y, angle } => { self.do_zoom(c, t, p, clip, zoom_x, zoom_y, angle); },
         };
    }

    pub fn default_color() -> Color {
        Color::RGBA(0, 0, 0, 255)
    }

    pub fn init(&self) -> &Self {
        self.clear(Self::default_color())
            .set_blend_mode(BlendMode::None)
            .emit();
        self
    }

}

impl Clone for RGTexture {

    fn clone(&self) -> Self {
        self.emit();
        let n = Self::create(self.canvas.clone(), self.texture_creator.clone(), self.width(), self.height());
        n.copy_plain(&self, None, Some(Rect::new(0, 0, self.width(), self.height())))
            .set_texture_alpha_mode(self.texture_alpha_mode())
            .set_draw_color(self.draw_color())
            .set_blend_mode(self.blend_mode())
            .emit();
        n
    }

}

pub mod operation;
pub mod property;
pub mod render;
pub mod log;
