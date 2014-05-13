use std::rc::Rc;

pub use sdl2::pixels;
use sdl2::render::Texture;
use sdl2_ttf;

use unit::Vec2;
use renderer::Renderer;
use sprite::Sprite;

pub struct Font {
    font: Rc<sdl2_ttf::Font>,
}

impl Font {
    pub fn from_file(renderer: &mut Renderer, filename: &str) -> Font {
        Font {
            font: renderer.load_font(filename),
        }
    }

    pub fn render(&self, renderer: &Renderer, text: &str, color: pixels::Color, at: Vec2) {
        let surface = self.font.render_str_blended(text, color).unwrap();
        let texture = renderer.sdl_renderer().create_texture_from_surface(&surface).unwrap();
        let sprite = Sprite::from_texture(Rc::<Texture>::new(texture));

        sprite.render(renderer, at);
    }
}
