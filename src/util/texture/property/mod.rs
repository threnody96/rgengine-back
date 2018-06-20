use super::RGTexture;
use super::operation::Operation;
use ::sdl2::pixels::Color;
use ::sdl2::render::BlendMode;

impl RGTexture {

    pub fn set_draw_color(&self, color: Color) -> &Self {
        self.regist(Operation::SetDrawColor { color: color });
        self
    }

    pub fn set_blend_mode(&self, mode: BlendMode) -> &Self {
        self.regist(Operation::SetBlendMode { mode: mode });
        self
    }

    pub fn set_alpha_mode(&self, alpha: u8) -> &Self {
        self.regist(Operation::SetAlphaMode { alpha: alpha });
        self
    }

}
