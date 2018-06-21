use super::RGTexture;
use super::operation::Operation;
use ::sdl2::pixels::Color;
use ::sdl2::render::{ Canvas, BlendMode };
use ::sdl2::video::Window;

impl RGTexture {

    pub fn set_default_property(&self) -> &Self {
        self.set_texture_alpha(255)
            .set_draw_color(Self::default_color())
            .set_blend_mode(BlendMode::None)
            .emit()
    }
    
    pub fn clone_property(&self, t: &RGTexture) -> &Self {
        self.set_texture_alpha(t.texture_alpha())
            .set_draw_color(t.draw_color())
            .set_blend_mode(t.blend_mode())
    }

    pub fn set_texture_alpha(&self, alpha: u8) -> &Self {
        self.emit().borrow_mut().set_alpha_mod(alpha);
        self
    }

    pub fn texture_alpha(&self) -> u8 {
        self.borrow().alpha_mod()
    }

    pub fn set_draw_color(&self, color: Color) -> &Self {
        self.log.borrow_mut().set_draw_color(color);
        self.regist(Operation::SetDrawColor { color: color });
        self
    }

    pub fn do_set_draw_color(&self, c: &mut Canvas<Window>, color: Color) {
        c.set_draw_color(color);
    }

    pub fn draw_color(&self) -> Color {
        self.log.borrow().draw_color().unwrap_or(Self::default_color())
    }

    pub fn set_blend_mode(&self, mode: BlendMode) -> &Self {
        self.log.borrow_mut().set_blend_mode(mode);
        self.regist(Operation::SetBlendMode { mode: mode });
        self
    }

    pub fn do_set_blend_mode(&self, c: &mut Canvas<Window>, mode: BlendMode) {
        c.set_blend_mode(mode);
    }

    pub fn blend_mode(&self) -> BlendMode {
        self.log.borrow().blend_mode().unwrap_or(self.texture.borrow().blend_mode())
    }

}
