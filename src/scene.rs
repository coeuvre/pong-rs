use std::vec::Vec;
use collections::deque::Deque;
use collections::ringbuf::RingBuf;

use unit::MS;
use input::Input;
use renderer::Renderer;

pub trait Scene {
    fn start(&mut self) -> bool;

    fn update(&mut self, scene_manager: &mut SceneManager, input: &Input, dt: MS);
    fn render(&self, renderer: &Renderer);

    fn end(&mut self);
}

pub struct SceneManager {
    scenes: Vec<~Scene>,
    actions: RingBuf<TransAction>,
}

enum TransAction {
    TRANS(~Scene),
    PUSH(~Scene),
    POP,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: Vec::<~Scene>::new(),
            actions: RingBuf::<TransAction>::new(),
        }
    }

    pub fn push(&mut self, scene: ~Scene) {
        self.actions.push_back(PUSH(scene));
    }

    pub fn pop(&mut self) {
        self.actions.push_back(POP);
    }

    pub fn trans(&mut self, scene: ~Scene) {
        self.actions.push_back(TRANS(scene));
    }

    pub fn update(&mut self, input: &Input, dt: MS) {
        match self.scenes.pop() {
            Some(mut s) => {
                s.update(self, input, dt);
                self.scenes.push(s);
            },
            None => {}
        }

        self.handle_trans();
    }

    pub fn render(&self, renderer: &Renderer) {
        match self.scenes.last() {
            Some(s) => s.render(renderer),
            None => {},
        }
    }

    fn handle_trans(&mut self) {
        'iter: loop {
            match self.actions.pop_front() {
                Some(action) => {
                    match action {
                        TRANS(s) => {
                            self.pop();
                            self.push(s);
                        },
                        PUSH(mut s) => {
                            if s.start() {
                                self.scenes.push(s);
                            }
                        },
                        POP => {
                            match self.scenes.pop() {
                                Some(mut s) => s.end(),
                                None => {},
                            }
                        },
                    }
                },
                None => {
                    break 'iter;
                },
            }
        }
    }
}
