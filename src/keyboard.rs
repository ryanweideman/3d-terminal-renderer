use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};

use std::time::Duration;

#[derive(PartialEq)]
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
    Right
}

pub struct Keyboard {
    pub pressed_keys : Vec<Keys>
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::new()
        }
    }

    pub fn update(&mut self) {
        if event::poll(Duration::from_millis(1)).expect("Failed to poll event") {
            match event::read().expect("Failed to read event") {
                Event::Key(key_event) => {
                    self.process_key_event(key_event);
                },
                _ => {}
            }
        }
    }

    fn process_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.pressed_keys.push(Keys::CtrlC);
                } else {
                    self.pressed_keys.push(Keys::C);
                }
            }
            KeyCode::Char('w') | KeyCode::Char('W') => self.pressed_keys.push(Keys::W),
            KeyCode::Char('s') | KeyCode::Char('S') => self.pressed_keys.push(Keys::S),
            KeyCode::Char('d') | KeyCode::Char('D') => self.pressed_keys.push(Keys::D),
            KeyCode::Char('a') | KeyCode::Char('A') => self.pressed_keys.push(Keys::A),
            KeyCode::Char(' ') => self.pressed_keys.push(Keys::Space),
            KeyCode::Left      => self.pressed_keys.push(Keys::Left),
            KeyCode::Right     => self.pressed_keys.push(Keys::Right),
            KeyCode::Up        => self.pressed_keys.push(Keys::Up),
            KeyCode::Down      => self.pressed_keys.push(Keys::Down),
            _ => {}
        }
    }

    pub fn clear_all_keys(&mut self) {
        self.pressed_keys.clear();
    }

    pub fn is_ctrl_pressed(&self) -> bool {
        self.pressed_keys.contains(&Keys::CtrlC)
    }
}