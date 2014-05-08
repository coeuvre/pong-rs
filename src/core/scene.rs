use std::vec::Vec;

use Core;

pub trait Scene {
    fn update(&mut self, core: &mut Core);
    fn render(&mut self, core: &mut Core);
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
        self.scenes.push(scene);
    }

    pub fn pop(&mut self) {
        self.scenes.pop();
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
