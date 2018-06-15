use std::cell::RefCell;
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::PixelFormatEnum::ARGB8888;
use ::sdl2::video::Window;
use super::component::operation::Operation;
use super::component::operation::OperationExecuter;

pub struct VirtualCanvas<'l, T> where T: 'l {
    canvas: &'l RefCell<Canvas<Window>>,
    vcanvas: RefCell<Texture<'l>>,
    texture_creator: &'l TextureCreator<T>,
}

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn new(canvas: &'l RefCell<Canvas<Window>>, texture_creator: &'l TextureCreator<T>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        Self::initialize_canvas(canvas);
        Self {
            canvas: canvas,
            vcanvas: RefCell::new(Self::create_new_vcanvas(&texture_creator, w, h)),
            texture_creator: texture_creator
        }
    }

    fn initialize_canvas(canvas: &'l RefCell<Canvas<Window>>) {
        let mut c = canvas.borrow_mut();
        c.set_blend_mode(BlendMode::Blend);
    }

    fn create_new_vcanvas(texture_creator: &'l TextureCreator<T>, width: u32, height: u32) -> Texture<'l> {
        let mut vcanvas = texture_creator.create_texture_target(ARGB8888, width, height).unwrap();
        vcanvas.set_blend_mode(BlendMode::Blend);
        vcanvas
    }

}

impl<'l, T> OperationExecuter<'l> for VirtualCanvas<'l, T> {

    fn operation_execute(&self, operation: &Operation<'l>) {
        match operation {
            Operation::Group { option, operations } => {
                self.sub_canvas(option.position, &|c| {
                    for o in operations { c.operation_execute(o); }
                });
            },
            Operation::Copy { t, p, clip } => {
                self.copy(t, p.clone(), clip.clone()).unwrap();
            },
            Operation::Zoom { t, p, clip, zoom_x, zoom_y } => {
                self.zoom(t, p.clone(), clip.clone(), zoom_x.clone(), zoom_y.clone()).unwrap();
            }
        }
    }

}

pub mod sub_canvas;
pub mod render;
