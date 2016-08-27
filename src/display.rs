extern crate ncurses;

use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use ncurses::*;
use messages::DisplayMsg;

fn update_display() {
    mvprintw(5,0,"Updating display example");
}

fn typewriter(y: i32, x: i32, message: &str) {
    mv(y,x);
    for c in message.chars() {
        addch(c as u64);
        thread::sleep(Duration::from_millis(80));
        refresh();
    }
    thread::sleep(Duration::from_millis(200));
}

fn main_intro() {
    typewriter(1,0, "*ominous music plays*");
    typewriter(2,0, "The year is 2016.");
    typewriter(3,0, "An election year.");
    typewriter(4,0, "The country is a cauldron of division, social unrest, memes, and internet tripe.");
    typewriter(5,0, "The entire media scrambles for ad revenue, uncertain whether to prop up a dying business model or reinvent it.");

    typewriter(8,0, "The only job you could find during the credit crisis was an unpaid internship at PooSprinkler,");
    typewriter(9,0, "a clickbait farm trying to claim a chunk of the attention-stealing cesspool.");
    typewriter(10,0, "You've now climbed to editor-in-chief, with only a meager budget at your disposal for content.");

    typewriter(11,0, "And then...tragedy strikes.");
    typewriter(12,0, "A noble beast is taken from his prime.");
    typewriter(13,0, "Duty calls...");
    typewriter(14,0, "Only you can capitalize on this with your dark arts of clickbait...to raise money for the zoo...");
    typewriter(16,0, "It's time to get...");
    thread::sleep(Duration::from_millis(1500));
    typewriter(17,0, "Clicks Out for Harambe.");
}

pub fn process_message(rx: Receiver<DisplayMsg>) {

    loop {
        match rx.recv().unwrap() {
            DisplayMsg::Time(s) => { mvprintw(1,75,&s);},
            DisplayMsg::UpdateDisplay => update_display(),
            DisplayMsg::Splash => splash(),
            DisplayMsg::MainIntro => main_intro(),
            DisplayMsg::EndThread => break,
        };
        refresh();
    }
}

fn splash() {
    mvprintw(0,0,"Clicks Out For Harambe!!!");
}

