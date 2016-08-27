extern crate ncurses;

use std::sync::mpsc::Receiver;
use ncurses::*;
use messages::DisplayMsg;

fn update_display() {
    mvprintw(5,0,"Updating display example");
}

pub fn process_message(rx: Receiver<DisplayMsg>) {

    loop {
        match rx.recv().unwrap() {
            DisplayMsg::Time(s) => { mvprintw(1,75,&s);},
            DisplayMsg::UpdateDisplay => update_display(),
            DisplayMsg::Splash => splash(),
            DisplayMsg::EndThread => break,
        };
        refresh();
    }
}

fn splash() {
    mvprintw(0,0,"Clicks Out For Harambe!!!");
}

