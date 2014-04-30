extern crate sdl2;
extern crate sdl2_mixer;
extern crate collections;

use std::rc::Rc;
use collections::hashmap::HashMap;

pub struct Mixer {
    musics: HashMap<~str, Rc<~sdl2_mixer::Music>>,
}

impl Mixer {
    pub fn new() -> Mixer {
        Mixer {
            musics: HashMap::<~str, Rc<~sdl2_mixer::Music>>::new(),
        }
    }

    pub fn load_music(&mut self, filename: ~str) -> Rc<~sdl2_mixer::Music> {
        self.musics.find_or_insert_with(filename, |key| {
            let path = &Path::new((*key).clone());
            Rc::new(sdl2_mixer::Music::from_file(path).unwrap())
        }).clone()
    }
}
