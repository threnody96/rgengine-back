use std::rc::Rc;
use std::cell::{ Ref, RefMut, RefCell };
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::{ Color, PixelFormatEnum };
use ::sdl2::video::{ Window, WindowContext };
use self::operation::Operation;

pub struct RGTexture {
    canvas: Rc<RefCell<Canvas<Window>>>,
    texture_creator: Rc<TextureCreator<WindowContext>>,
    texture: RefCell<Texture>,
    operations: RefCell<Vec<Operation>>
}

impl RGTexture {

    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>, texture: RefCell<Texture>) -> Self {
        Self {
            canvas: canvas,
            texture_creator: texture_creator,
            texture: texture,
            operations: RefCell::new(Vec::new())
        }
    }

    pub fn create(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>, width: u32, height: u32) -> Self {
        let o = texture_creator.create_texture_target(PixelFormatEnum::ARGB8888, width, height).unwrap();
        Self::new(canvas.clone(), texture_creator.clone(), RefCell::new(o))
    }

    pub fn borrow(&self) -> Ref<Texture> { self.texture.borrow() }

    pub fn borrow_mut(&self) -> RefMut<Texture> { self.texture.borrow_mut() }

    pub fn width(&self) -> u32 { self.borrow().query().width }

    pub fn height(&self) -> u32 { self.borrow().query().height }

    fn search_last_operation(&self, operation: Operation) -> Option<Operation> {
        for o in self.operations.borrow().iter().rev() {
            if o.id() == operation.id() { return Some(o.clone()) }
        }
        None
    }

    fn regist(&self, operation: Operation) {
        self.operations.borrow_mut().push(operation);
    }

}

impl Clone for RGTexture {

    fn clone(&self) -> Self {
        let n = Self::create(self.canvas.clone(), self.texture_creator.clone(), self.width(), self.height());
        n
    }

}

pub mod operation;
pub mod property;
