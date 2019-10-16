extern crate sdl2;

use std::collections::HashMap;

pub struct Keyboard {
    pub keymap: HashMap<sdl2::keyboard::Keycode, usize>,
}

impl Keyboard {
    pub fn new() -> Self {
        let mut keys: HashMap<sdl2::keyboard::Keycode, usize> = HashMap::new();
        keys.insert(sdl2::keyboard::Keycode::Num1, 0x1 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num2, 0x2 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num3, 0x3 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num4, 0x4 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num5, 0x5 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num6, 0x6 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num7, 0x7 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num8, 0x8 as usize);
        keys.insert(sdl2::keyboard::Keycode::Num9, 0x9 as usize);
        keys.insert(sdl2::keyboard::Keycode::A, 0xA as usize);
        keys.insert(sdl2::keyboard::Keycode::B, 0xB as usize);
        keys.insert(sdl2::keyboard::Keycode::C, 0xC as usize);
        keys.insert(sdl2::keyboard::Keycode::D, 0xD as usize);
        keys.insert(sdl2::keyboard::Keycode::E, 0xE as usize);
        keys.insert(sdl2::keyboard::Keycode::F, 0xF as usize);
        Keyboard { keymap: keys }
    }
}
