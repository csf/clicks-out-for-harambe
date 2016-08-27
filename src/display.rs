extern crate ncurses;

use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use ncurses::*;
use messages::DisplayMsg;

fn typewriter(y: i32, x: i32, message: &str) {
    mv(y,x);
    for c in message.chars() {
        addch(c as u64);
        thread::sleep(Duration::from_millis(62));
        refresh();
    }
    thread::sleep(Duration::from_millis(240));
}

fn main_intro() {
    typewriter(1,0, "*ominous music plays*");
    typewriter(2,0, "The year is 2016.");
    typewriter(3,0, "An election year.");
    typewriter(4,0, "The country is a cauldron of division, social unrest, memes, and internet tripe.");
    thread::sleep(Duration::from_millis(500));
    typewriter(6,0, "The media industry scrambles for ad revenue, uncertain whether to prop up a dying business model or reinvent it.");

    typewriter(7,0, "The only gig you could find during the credit crisis: an unpaid internship at PooSprinkler,");
    typewriter(8,0, "a clickbait site turning the grist of listicles into pennies of ad revenue.");
    typewriter(10,0, "Through years of toil in the content mines, you've become editor-in-chief, with a meager content budget at your disposal.");

    typewriter(12,0, "And then...tragedy strikes our nation.");
    thread::sleep(Duration::from_millis(500));
    typewriter(13,0, "A noble beast is taken from his prime.");
    thread::sleep(Duration::from_millis(500));
    typewriter(14,0, "Duty calls...");
    typewriter(15,0, "Only your mastery of the dark arts of clickbait can save your site from irrelevance...");
    thread::sleep(Duration::from_millis(500));
    typewriter(17,0, "It's time to get...");
    thread::sleep(Duration::from_millis(2500));
    clear();
    splash();
}

pub fn process_message(rx: Receiver<DisplayMsg>) {

    loop {
        match rx.recv().unwrap() {
            DisplayMsg::MainIntro => main_intro(),
            DisplayMsg::Tutorial => tutorial(),
            DisplayMsg::InitialScreen(time) => { clear(); update_screen(time);},
            //DisplayMsg::EndThread => break,
        };
        refresh();
    }
}



fn splash() {
    mvprintw(0,0,"Clicks Out For Harambe!!!");
}

fn tutorial() {
    clear();
    mvprintw(0,0, "Tutorial placeholder");
}

fn draw_time(time: u64) {
    mvprintw(0,75,&format!("{:?}",time));
}

fn update_screen(time: u64) {
    draw_time(time);
}
