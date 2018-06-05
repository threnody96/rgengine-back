extern crate sdl2;

use self::sdl2::render::{BlendMode, Canvas};
use self::sdl2::video::Window;

pub trait Operation {
    
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;

}
