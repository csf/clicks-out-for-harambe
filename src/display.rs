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
        refresh();
        match c {
            ',' | ':' => thread::sleep(Duration::from_millis(190)),
            _ => thread::sleep(Duration::from_millis(62)),
        }
    }
    thread::sleep(Duration::from_millis(240));
}

fn anykey() {
    mv(24,24);
    printw("Press any key to continue...");
    refresh();
}

fn main_intro() {
    clear();
    refresh();
    typewriter(1,0, "*ominous music plays*");
    typewriter(2,0, "The year is 2016.");
    typewriter(3,0, "An election year.");
    typewriter(4,0, "The country is a sewer of division, social unrest, dank memes, and internet tripe.");
    thread::sleep(Duration::from_millis(500));
    typewriter(6,0, "The media industry scrambles for ad revenue, uncertain whether to prop up a dying business model, or reinvent it.");

    typewriter(7,0, "The only gig you could find during the credit crisis: an unpaid internship at PooSprinkler,");
    typewriter(8,0, "a clickbait site turning the grist of listicles into a feeble stream of ad revenue.");
    typewriter(10,0, "Through years of toil in the content mines, you've become editor-in-chief, only a meager content budget at your disposal.");

    typewriter(12,0, "And then...");
    thread::sleep(Duration::from_millis(500));
    typewriter(12,11, "tragedy strikes our nation.");
    thread::sleep(Duration::from_millis(500));
    typewriter(13,0, "A noble beast is taken from his prime.");
    thread::sleep(Duration::from_millis(500));
    typewriter(15,0, "Harambe has been assassinated!");
    thread::sleep(Duration::from_millis(500));
    typewriter(17,0, "No time to mourn...duty calls.");
    thread::sleep(Duration::from_millis(500));
    typewriter(18,0, "Only your mastery of the dark arts of clickbait can save your site from irrelevance...");
    thread::sleep(Duration::from_millis(500));
    typewriter(20,0, "It's time to get...");
    thread::sleep(Duration::from_millis(2500));
    clear();
    refresh();
    splash();
    thread::sleep(Duration::from_millis(2000));
}

pub fn process_message(rx: Receiver<DisplayMsg>) {

    loop {
        match rx.recv().unwrap() {
            DisplayMsg::MainIntro => main_intro(),
            DisplayMsg::Tutorial => tutorial(),
            DisplayMsg::InitialScreen(time) => { clear(); refresh(); update_screen(time);},
            DisplayMsg::AnyKey => anykey(),
            DisplayMsg::AnyKeyPause => { 
                                        thread::sleep(Duration::from_millis(2000));
                                        anykey();
            },
            //DisplayMsg::EndThread => break,
        };
        refresh();
    }
}

fn splash() {
    mvprintw(0,0,"Clicks Out For Harambe!!!");
    refresh();
}

fn tutorial() {
    clear();
    refresh();
    mvprintw(0,0, "Tutorial placeholder");
}

fn draw_time(time: u64) {
    mvprintw(0,75,&format!("{:?}",time));
}

fn update_screen(time: u64) {
    draw_time(time);
}
