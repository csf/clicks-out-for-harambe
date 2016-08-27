extern crate ncurses;

mod messages;
mod display;
mod input_handling;

use ncurses::*;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
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
    clear();

    // Main intro
    disp_tx.send(DisplayMsg::MainIntro).unwrap();
   
    thread::sleep(Duration::from_millis(4000));
    
    getch();

    disp_tx.send(DisplayMsg::Tutorial).unwrap();

    getch();

    disp_tx.send(DisplayMsg::InitialScreen(1440)).unwrap();
    
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
