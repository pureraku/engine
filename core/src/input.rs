use glam::Vec2;
use glfw::{Action, Key, MouseButton};
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct Input {
    keys_down: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,

    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    pub mouse_pos: Vec2,
    pub last_mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: Vec2,
    first_mouse: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_down: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: Vec2::ZERO,
            first_mouse: true,
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    pub fn on_key(&mut self, key: Key, action: Action) {
        match action {
            Action::Press => {
                self.keys_down.insert(key);
                self.keys_pressed.insert(key);
            }
            Action::Release => {
                self.keys_down.remove(&key);
                self.keys_released.insert(key);
            }
            _ => {}
        }
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, action: Action) {
        match action {
            Action::Press => {
                self.mouse_down.insert(button);
                self.mouse_pressed.insert(button);
            }
            Action::Release => {
                self.mouse_down.remove(&button);
                self.mouse_released.insert(button);
            }
            _ => {}
        }
    }

    pub fn on_cursor_pos(&mut self, x: f64, y: f64) {
        let current = Vec2::new(x as f32, y as f32);
        if self.first_mouse {
            self.last_mouse_pos = current;
            self.first_mouse = false;
        }
        self.mouse_delta += current - self.last_mouse_pos;
        self.last_mouse_pos = current;
        self.mouse_pos = current;
    }

    pub fn on_scroll(&mut self, x: f64, y: f64) {
        self.scroll_delta += Vec2::new(x as f32, y as f32);
    }

    pub fn reset_mouse(&mut self) {
        self.first_mouse = true;
        self.mouse_delta = Vec2::ZERO;
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        self.mouse_delta = Vec2::ZERO;
        self.scroll_delta = Vec2::ZERO;
    }
}
