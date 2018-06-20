use std::rc::Rc;
use ::sdl2::render::Texture;
use ::sdl2::pixels::{ Color };
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::BlendMode;

#[derive(Clone)]
pub enum Operation {
    SetDrawColor { color: Color },
    SetBlendMode { mode: BlendMode },
    SetAlphaMode { alpha: u8 },
    FillRect { color: Color, rect: Rect },
    Clear { color: Color },
    Copy { t: Rc<Texture>, p: Point, clip: Option<Rect>, angle: f64 },
    Zoom { t: Rc<Texture>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64 }
}

impl Operation {

    pub fn id(&self) -> u32 {
        match self {
            Operation::SetDrawColor { color }      => 1,
            Operation::SetBlendMode { mode }       => 2,
            Operation::SetAlphaMode { alpha }      => 3,
            Operation::FillRect { color, rect }    => 4,
            Operation::Clear { color }             => 5,
            Operation::Copy { t, p, clip, angle }  => 6,
            Operation::Zoom { t, p, clip, zoom_x, zoom_y, angle } => {
                7
            }
        }
    }

}

