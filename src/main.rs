#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::{
    sync::mpsc::{RecvTimeoutError, channel},
    thread,
    time::Duration,
};

use getch::Getch;

use crate::timer::Timer;

const LOOP_DELAY: Duration = Duration::from_millis(10);
const ESC_KEY: u8 = 27;

//use owo_colors::OwoColorize;
pub mod scramble;
pub mod timer;

fn main() {

    let mut timer = Timer::default();
    let (sender, receiver) = channel();

    thread::spawn(move || {
        let getch = Getch::new();
        loop {
            let pressed_key = getch.getch().unwrap();
            if pressed_key == ESC_KEY {
                drop(sender);
                break;
            }
            sender.send(pressed_key).unwrap();
        }
    });

    loop {
        
        let maybe_key = match receiver.recv_timeout(LOOP_DELAY) {
            Ok(key) => Some(key),
            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
            Err(RecvTimeoutError::Timeout) => None,
        };

        if let Some(key) = maybe_key {
            timer.handle_keypress();
        } else {
            timer.tick();   
        }

    }
}
