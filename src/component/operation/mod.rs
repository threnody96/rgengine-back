use std::rc::Rc;
use super::ComponentOption;
use ::util::texture::RGTexture;
use ::sdl2::render::Texture;
use ::sdl2::rect::{ Point, Rect };

pub trait OperationExecuter {
    fn operation_execute(&self, operation: &Operation);
}

#[derive(Clone)]
pub enum Operation {
    Group { option: ComponentOption, operations: Vec<Operation> },
    Copy { t: Rc<RGTexture>, p: Point, clip: Option<Rect>, angle: f64 },
    Zoom { t: Rc<RGTexture>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64 }
}

