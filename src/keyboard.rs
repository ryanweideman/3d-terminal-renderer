use std::collections::HashSet;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Eq, Hash, PartialEq)]
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

    pub fn update(&mut self) -> io::Result<()> {
        self.pressed_keys.clear();
        // Process and clear the crossterm event buffer
        const MAX_POLL_COUNT: usize = 1000;
        for _ in 0..MAX_POLL_COUNT {
            if event::poll(Duration::from_millis(0))? {
                if let Event::Key(key_event) = event::read()? {
                    self.process_key_event(key_event);
                }
            }
        }
        Ok(())
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
}
