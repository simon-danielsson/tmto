use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll};

use crate::{State, Tmto};

impl Tmto {
    pub fn controls(&mut self) -> std::io::Result<()> {
        if poll(Duration::ZERO)? {
            match self.state {
                State::Active => {
                    if let Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) = event::read()?
                    {
                        match (code, modifiers) {
                            // quit
                            (KeyCode::Esc, _) => {
                                self.state = State::Quit;
                            }

                            (
                                KeyCode::Char('c'),
                                KeyModifiers::CONTROL,
                            ) => {
                                self.state = State::Quit;
                            }
                            _ => {
                                self.state = State::Quit;
                            }
                        }
                    }
                }

                State::Quit => {}
            }
        }
        Ok(())
    }
}
