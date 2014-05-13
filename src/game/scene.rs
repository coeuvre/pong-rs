use std::vec::Vec;

use Game;

pub trait Scene {
    fn update(&mut self, game: &mut Game);
    fn render(&mut self, game: &mut Game);
}

pub struct SceneManager {
    scenes: Vec<Box<Scene>>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: Vec::<Box<Scene>>::new(),
        }
    }

    pub fn push(&mut self, scene: Box<Scene>) {
        self.scenes.push(scene);
    }

    pub fn pop(&mut self) {
        self.scenes.pop();
    }

    pub fn trans(&mut self, scene: Box<Scene>) {
        self.pop();
        self.push(scene);
    }

    pub fn update(&mut self, game: &mut Game) {
        match self.scenes.mut_last() {
            Some(s) => {
                s.update(game);
            },
            None => {}
        }
    }

    pub fn render(&mut self, game: &mut Game) {
        match self.scenes.mut_last() {
            Some(s) => {
                s.render(game);
            },
            None => {},
        }
    }
}
