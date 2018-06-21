use ::sdl2::pixels::{ Color };
use ::sdl2::render::BlendMode;

pub struct OperationLog {
    draw_color: Option<Color>,
    blend_mode: Option<BlendMode>,
}

impl OperationLog {
    
    pub fn new() -> Self {
        Self {
            draw_color: None,
            blend_mode: None
        }
    }

    pub fn set_draw_color(&mut self, color: Color) { self.draw_color = Some(color); }

    pub fn draw_color(&self) -> Option<Color> { self.draw_color }

    pub fn set_blend_mode(&mut self, mode: BlendMode) { self.blend_mode = Some(mode); }

    pub fn blend_mode(&self) -> Option<BlendMode> { self.blend_mode }

}
