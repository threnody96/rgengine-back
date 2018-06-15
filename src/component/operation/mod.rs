use std::rc::Rc;
use super::ComponentOption;
use ::sdl2::render::Texture;
use ::sdl2::rect::{ Point, Rect };

pub trait OperationExecuter<'l> {
    fn operation_execute(&self, operation: &Operation<'l>);
}

#[derive(Clone)]
pub enum Operation<'l> {
    Group { option: ComponentOption, operations: Vec<Operation<'l>> },
    Copy { t: Rc<Texture<'l>>, p: Point, clip: Option<Rect> },
    Zoom { t: Rc<Texture<'l>>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32> }
}

