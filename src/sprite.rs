use std::rc::Rc;

use sdl2::render::Texture;

use renderer::Renderer;

use unit::{Pixel, ToPixel, Point, Size, Vec2};

pub struct Sprite {
    texture: Rc<~Texture>,

    point: Point,
    size: Size,

    pivot: Point,
}

impl Sprite {
    pub fn new(renderer: &mut Renderer, filename: ~str) -> Sprite {
        let tex = renderer.load_texture(filename);
        let query = tex.query().unwrap();
        let size = Size::new(Pixel(query.width), Pixel(query.height));
        Sprite {
            texture: tex,
            point: Point::new(Pixel(0), Pixel(0)),
            size: size,
            pivot: Point::new(Pixel(0), Pixel(0)),
        }
    }

    pub fn render(&self, renderer: &Renderer, at: Vec2) {
        let mut at = Point::new(at.x.to_pixel(), at.y.to_pixel());
        at = at - self.pivot;
        renderer.blit(self.texture.clone(), self.point, self.size, at);
    }

    pub fn set_pivot(&mut self, pivot: Point) {
        self.pivot = pivot;
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
