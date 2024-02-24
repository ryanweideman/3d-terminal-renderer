use std::collections::HashSet;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Eq, Hash, PartialEq)]
pub enum Keys {
    W,
    A,
    S,
    D,
    C,
    Space,
    CtrlC,
    Up,
    Down,
    Left,
    Right,
}

pub struct Keyboard {
    pub pressed_keys: HashSet<Keys>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self) {
        if event::poll(Duration::from_millis(10)).expect("Failed to poll event") {
            if let Event::Key(key_event) = event::read().expect("Failed to read event") {
                self.process_key_event(key_event);
            }
        }
    }

    fn process_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.pressed_keys.insert(Keys::CtrlC);
                } else {
                    self.pressed_keys.insert(Keys::C);
                }
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.pressed_keys.insert(Keys::W);
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.pressed_keys.insert(Keys::S);
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.pressed_keys.insert(Keys::D);
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.pressed_keys.insert(Keys::A);
            }
            KeyCode::Char(' ') => {
                self.pressed_keys.insert(Keys::Space);
            }
            KeyCode::Left => {
                self.pressed_keys.insert(Keys::Left);
            }
            KeyCode::Right => {
                self.pressed_keys.insert(Keys::Right);
            }
            KeyCode::Up => {
                self.pressed_keys.insert(Keys::Up);
            }
            KeyCode::Down => {
                self.pressed_keys.insert(Keys::Down);
            }
            _ => {}
        }
    }

    pub fn clear_all_keys(&mut self) {
        self.pressed_keys.clear();
    }

    pub fn is_ctrl_c_pressed(&self) -> bool {
        self.pressed_keys.contains(&Keys::CtrlC)
    }
}
