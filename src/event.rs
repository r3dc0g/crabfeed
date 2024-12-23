use crate::time::{SystemTimeTick, Tick, TIME_STEP, TIME_STEP_MILLIS};
use crate::AppResult;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use std::{sync::mpsc, thread};

pub enum TerminalEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick { tick: Tick },
}

pub struct EventHandler {
    receiver: mpsc::Receiver<TerminalEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        {
            let sender = sender.clone();
            let mut last_tick = Tick::now();
            thread::spawn(move || loop {
                if event::poll(TIME_STEP).expect("no events available") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(key) => {
                            if key.kind == KeyEventKind::Press {
                                sender.send(TerminalEvent::Key(key))
                            } else {
                                Ok(())
                            }
                        }
                        CrosstermEvent::Mouse(e) => sender.send(TerminalEvent::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(TerminalEvent::Resize(w, h)),
                        _ => Ok(()),
                    }
                    .expect("unable to send event");
                }

                let now = Tick::now();
                if now - last_tick >= TIME_STEP_MILLIS {
                    if let Err(_) = sender.send(TerminalEvent::Tick { tick: now }) {
                        break;
                    }
                    last_tick = now;
                }
            })
        };

        Self { receiver }
    }

    pub fn next(&self) -> AppResult<TerminalEvent> {
        Ok(self.receiver.recv()?)
    }
}
