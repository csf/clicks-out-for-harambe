extern crate ncurses;

mod messages;
mod display;
mod input_handling;

use ncurses::*;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::Receiver;
use display::*;
use input_handling::{input_handler};
use messages::{MainLoopMsg,DisplayMsg};

fn main() {
    initscr();
    refresh();

    // Spawn the display processing thread
    let (disp_tx, disp_rx) = channel();
    thread::spawn(move|| {
        process_message(disp_rx);
    });
   
    //test
    disp_tx.send(DisplayMsg::Time("24:00".to_string())).unwrap();
    disp_tx.send(DisplayMsg::Splash).unwrap();
    disp_tx.send(DisplayMsg::UpdateDisplay).unwrap();
   
    let (input_tx, input_rx) = channel();
    thread::spawn(move|| {
        input_handler(input_tx);
    });

    // wait for any key to terminate
    loop {
        match input_rx.recv().unwrap() {
            MainLoopMsg::Quit => break,
        }
    }

    endwin();
}
