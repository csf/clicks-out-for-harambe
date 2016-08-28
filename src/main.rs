extern crate ncurses;

mod messages;
mod display;
mod input_handling;

mod gamestate;

use ncurses::*;
use std::sync::mpsc::channel;
use std::thread;
use display::*;
use input_handling::{input_handler};
use messages::{MainLoopMsg,DisplayMsg};

use gamestate::State;

fn main() {
    initscr();
    clear();
    noecho();
    refresh();

    // Spawn the display processing thread
    let (disp_tx, disp_rx) = channel();
    thread::spawn(move|| {
        process_message(disp_rx);
    });

    // set up game state
    let mut gamestate = State::new();

    // Main intro
    disp_tx.send(DisplayMsg::MainIntro).unwrap();
   
    disp_tx.send(DisplayMsg::AnyKeyPause).unwrap();
   
    getch();

    disp_tx.send(DisplayMsg::Tutorial).unwrap();
    
    disp_tx.send(DisplayMsg::AnyKeyPause).unwrap();
    
    getch();

    disp_tx.send(DisplayMsg::InitialScreen(gamestate)).unwrap();
    
    let (input_tx, input_rx) = channel();
    thread::spawn(move|| {
        input_handler(input_tx);
    });

    // wait for any key to terminate
    loop {
        //update_game_time();

        match input_rx.recv().unwrap() {
            MainLoopMsg::Quit => break,
        }
    }

    endwin();
}
