extern crate ncurses;

mod messages;
mod display;
mod feature;
mod gamestate;

use ncurses::*;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use messages::*;

use gamestate::{State,Story};

fn countdown(tx: Sender<MainLoopMsg>) {
    loop {
        thread::sleep(Duration::from_millis(1000));
        tx.send(MainLoopMsg::Tick).unwrap();
    }
}

fn init_test_gamestate(gamestate: &mut State) {
    // some test stories
    let story = Story::new("Gorilla Isn't Mist".to_string(), 525);
    gamestate.publish(story);
    let story = Story::new("A Nation Mourns".to_string(), 525);
    
    gamestate.publish(story);
}

fn main() {

    // set up game state
    let mut gamestate = State::new();
    init_test_gamestate(&mut gamestate);

    display::init_ncurses();

    // Title sequence stuff
    display::main_intro();
    display::tutorial();

    // we want to get full commands now, while updating display, so we need to use halfdelay+echo
    display::change_input_mode();

    // Paint the starting screen now
    display::update_screen(&gamestate);
    
    // spawn a thread to handle the game timer: this will refresh screen once a second 
    let (mainloop_tx, mainloop_rx) = channel();
    thread::spawn(move|| {
        countdown(mainloop_tx);
    });

    // Check for keyboard input
    // We keep current "draft" headline in gamestate so it can be retained and
    // refreshed if tick occurs
    loop {
        let ch = getch();
        if ch !=ERR {
            if ch != '\r' as i32 && ch != '\n' as i32 {
                unsafe { gamestate.draft_headline.as_mut_vec().push(ch as u8); }
            } else {
                // update game state by publishing new story
                let story = Story::new(gamestate.draft_headline.clone(), 423);
                gamestate.publish(story);
                gamestate.draft_headline = String::new();
                display::update_screen(&gamestate);
            }
        }

        // no typing, so channel check to see if it is time to refresh 
        match mainloop_rx.try_recv() {
            Ok(msg) => {
                match msg {
                    MainLoopMsg::Tick => {
                        gamestate.tick();
                        display::update_screen(&gamestate);
                        if gamestate.seconds_remaining == 0 {
                            break; // for now
                        }
                    },
                }
            },
            Err(_) => {},
        }
    }

    endwin();
}
