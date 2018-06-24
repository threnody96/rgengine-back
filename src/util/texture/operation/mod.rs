use std::rc::Rc;
use super::RGTexture;
use ::sdl2::pixels::{ Color };
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::BlendMode;

#[derive(Clone)]
pub enum Operation {
    SetDrawColor { color: Color },
    SetBlendMode { mode: BlendMode },
    FillRect { color: Option<Color>, rect: Rect },
    Clear { color: Option<Color> },
    Copy { t: Rc<RGTexture>, p: Point, clip: Option<Rect>, angle: f64 },
    Zoom { t: Rc<RGTexture>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64 }
}

