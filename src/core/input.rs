use sdl2::keycode;

use collections::hashmap::HashMap;

pub struct Input {
    pressed_keys: HashMap<keycode::KeyCode, bool>,
    released_keys: HashMap<keycode::KeyCode, bool>,
    held_keys: HashMap<keycode::KeyCode, bool>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            pressed_keys: HashMap::<keycode::KeyCode, bool>::new(),
            released_keys: HashMap::<keycode::KeyCode, bool>::new(),
            held_keys: HashMap::<keycode::KeyCode, bool>::new(),
        }
    }

    pub fn begin_new_frame(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    pub fn key_down_event(&mut self, keycode: keycode::KeyCode) {
        self.pressed_keys.insert(keycode, true);
        self.held_keys.insert(keycode, true);
    }

    pub fn key_up_event(&mut self, keycode: keycode::KeyCode) {
        self.released_keys.insert(keycode, true);
        self.held_keys.insert(keycode, false);
    }

    pub fn is_key_pressed(&self, keycode: keycode::KeyCode) -> bool {
        match self.pressed_keys.find(&keycode) {
            Some(state) => *state,
            None => false,
        }
    }

    pub fn is_key_released(&self, keycode: keycode::KeyCode) -> bool {
        match self.released_keys.find(&keycode) {
            Some(state) => *state,
            None => false,
        }
    }

    pub fn is_key_held(&self, keycode: keycode::KeyCode) -> bool {
        match self.held_keys.find(&keycode) {
            Some(state) => *state,
            None => false,
        }
    }
}
