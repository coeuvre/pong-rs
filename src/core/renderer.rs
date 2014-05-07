use std::rc::Rc;
use collections::hashmap::HashMap;

use sdl2::rect;

use sdl2::video;
use sdl2::video::Window;

use sdl2::render;
use sdl2::render::Texture;

use sdl2_image::LoadTexture;

use unit::{Point, Size};

pub struct Renderer {
    textures: ~HashMap<~str, Rc<~Texture>>,
    renderer: ~render::Renderer,
}

impl Renderer {
    pub fn new(title: &str, size: Size) -> Renderer {
        let w = size.w.val();
        let h = size.h.val();

        let window = Window::new(
            title, video::PosCentered, video::PosCentered,
            w, h, video::OpenGL
        ).unwrap();

        let renderer = render::Renderer::from_window(
            window, render::DriverAuto, render::Accelerated
        ).unwrap();

        Renderer {
            textures: ~HashMap::<~str, Rc<~Texture>>::new(),
            renderer: renderer,
        }
    }


    pub fn load_texture(&mut self, filename: ~str) -> Rc<~Texture> {
        let self_renderer = &self.renderer;

        self.textures.find_or_insert_with(filename, |key| {
            let path = Path::new((*key).clone());
            Rc::new(self_renderer.load_texture(&path).unwrap())
        }).clone()
    }

    pub fn blit(&self, texture: Rc<~Texture>,
                point: Point, size: Size,
                at: Point) {
        let x = point.x.val() as i32;
        let y = point.y.val() as i32;
        let w = size.w.val() as i32;
        let h = size.h.val() as i32;
        let dx = at.x.val() as i32;
        let dy = at.y.val() as i32;
        let source_rect = rect::Rect::new(x, y, w, h);
        let destination_rect = rect::Rect::new(dx, dy, w, h);

        match self.renderer.copy(*texture,
                                 Some(source_rect),
                                 Some(destination_rect)) {
            Ok(()) => {},
            Err(e) => println!("Warning: {}", e),
        }
    }

    pub fn clear(&self) {
        match self.renderer.clear() {
            Ok(()) => {},
            Err(e) => println!("Warning: {}", e),
        }
    }

    pub fn swap(&self) {
        self.renderer.present();
    }
}
