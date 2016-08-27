extern crate ncurses;

use std::sync::mpsc::Sender;
use ncurses::getch;
use messages::MainLoopMsg;

pub fn input_handler(tx: Sender<MainLoopMsg>) {
    getch();
    tx.send(MainLoopMsg::Quit).unwrap();
}

