extern crate ncurses;

mod messages;
mod display;
mod input_handling;

mod gamestate;

use ncurses::*;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use display::*;
use input_handling::{input_handler};
use messages::{MainLoopMsg,DisplayMsg};

use gamestate::{State,Story};

fn countdown(tx: Sender<MainLoopMsg>) {
    loop {
        thread::sleep(Duration::from_millis(1000));
        tx.send(MainLoopMsg::Tick).unwrap();
    }
}

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

    // some test stories
    let story = Story::new("Gorilla Isn't Mist".to_string(), 525);
    gamestate.publish(story);
    let story = Story::new("A Nation Mourns".to_string(), 525);
    
    gamestate.publish(story);

    // Main intro
    disp_tx.send(DisplayMsg::MainIntro).unwrap();
   
    disp_tx.send(DisplayMsg::AnyKeyPause).unwrap();
   
    getch();

    disp_tx.send(DisplayMsg::Tutorial).unwrap();
    
    disp_tx.send(DisplayMsg::AnyKeyPause).unwrap();
    
    getch();

    disp_tx.send(DisplayMsg::InitialScreen(gamestate.clone())).unwrap();
    
    let (mainloop_tx, mainloop_rx) = channel();
    
    // Create 
    let t1 = mainloop_tx.clone();
    let t2 = mainloop_tx.clone();

    // spawn a thread to handle input
    thread::spawn(move|| {
        input_handler(t1);
    });

    // spawn a thread to handle the game timer 
    thread::spawn(move|| {
        countdown(t2);
    });

    // Advance one clock tick, update game state, and handle any input events.
    loop {
        match mainloop_rx.recv().unwrap() {
            MainLoopMsg::Tick => {
                gamestate.tick();
                disp_tx.send(DisplayMsg::UpdateScreen(gamestate.clone())).unwrap();
            },
            MainLoopMsg::Quit => break,
        }
    }

    endwin();
}
