use std::vec::Vec;

use Core;

pub trait Scene {
    fn start(&mut self) -> bool;

    fn update(&mut self, core: &mut Core);
    fn render(&mut self, core: &mut Core);

    fn end(&mut self);
}

pub struct SceneManager {
    scenes: Vec<~Scene>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: Vec::<~Scene>::new(),
        }
    }

    pub fn push(&mut self, scene: ~Scene) {
        let mut s = scene;
        if s.start() {
            self.scenes.push(s);
        }
    }

    pub fn pop(&mut self) {
        match self.scenes.pop() {
            Some(mut scene) => scene.end(),
            None => {},
        }
    }

    pub fn trans(&mut self, scene: ~Scene) {
        self.pop();
        self.push(scene);
    }

    pub fn update(&mut self, core: &mut Core) {
        match self.scenes.mut_last() {
            Some(s) => {
                s.update(core);
            },
            None => {}
        }
    }

    pub fn render(&mut self, core: &mut Core) {
        match self.scenes.mut_last() {
            Some(s) => {
                s.render(core);
            },
            None => {},
        }
    }
}
