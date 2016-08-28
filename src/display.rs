extern crate ncurses;

use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use ncurses::*;
use messages::DisplayMsg;
use gamestate::{State,Story};

#[cfg(feature="skip_intro")]
const KEY_DELAY: u64 = 0;
#[cfg(feature="skip_intro")]
const COMMA_DELAY: u64 = 0;
#[cfg(feature="skip_intro")]
const LINE_DELAY: u64 = 0;
#[cfg(feature="skip_intro")]
const DRAMA_DELAY: u64 = 0;

#[cfg(not(feature="skip_intro"))]
const KEY_DELAY: u64 = 62;
#[cfg(not(feature="skip_intro"))]
const COMMA_DELAY: u64 = 190;
#[cfg(not(feature="skip_intro"))]
const LINE_DELAY: u64 = 240;
#[cfg(not(feature="skip_intro"))]
const DRAMA_DELAY: u64 = 500;


fn typewriter(y: i32, x: i32, message: &str) {
    mv(y,x);
    for c in message.chars() {
        addch(c as u64);
        refresh();
        match c {
            ',' | ':' => thread::sleep(Duration::from_millis(COMMA_DELAY)),
            _ => thread::sleep(Duration::from_millis(KEY_DELAY)),
        }
    }
    thread::sleep(Duration::from_millis(LINE_DELAY));
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
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(6,0, "The media industry scrambles for ad revenue, uncertain whether to prop up a dying business model, or reinvent it.");

    typewriter(7,0, "The only gig you could find during the credit crisis: an unpaid internship at PooSprinkler,");
    typewriter(8,0, "a clickbait site turning the grist of listicles into a feeble stream of ad revenue.");
    typewriter(10,0, "Through years of toil in the content mines, you've become editor-in-chief, only a meager story budget at your disposal.");

    typewriter(12,0, "And then...");
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(12,11, "tragedy strikes our nation.");
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(13,0, "A noble beast is taken from his prime.");
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(15,0, "Harambe has been assassinated!");
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(17,0, "No time to mourn...duty calls.");
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(18,0, "Only your mastery of the dark arts of clickbait can save your site from irrelevance...");
    thread::sleep(Duration::from_millis(DRAMA_DELAY));
    typewriter(20,0, "It's time to go...");
    thread::sleep(Duration::from_millis(5*DRAMA_DELAY));
    clear();
    refresh();
    splash();
    thread::sleep(Duration::from_millis(4*DRAMA_DELAY));
}

pub fn process_message(rx: Receiver<DisplayMsg>) {

    loop {
        match rx.recv().unwrap() {
            DisplayMsg::MainIntro => main_intro(),
            DisplayMsg::Tutorial => tutorial(),
            DisplayMsg::InitialScreen(state) => {
                clear();
                refresh();
                update_screen(state);
            },
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

fn draw_budget(budget: f64) {
    mvprintw(0,0,&format!("Budget: {:.*}", 2, budget));
}

fn draw_time(time: u64) {
    mvprintw(0,75,&format!("{:?}",time));
}

fn draw_stories(stories: Vec<Story>) {
    let mut queue_line = 2;
    attron(A_BOLD());
    mvprintw(queue_line,0,"Your Top Headlines");
    attroff(A_BOLD());
    queue_line += 1;
    attron(A_UNDERLINE());
    mvprintw(queue_line,0," Pos   Revenue    Headline");
    attroff(A_UNDERLINE());
    queue_line += 1;

    let mut story_pos = 1;
    for s in stories {
        queue_line += 1;
        story_pos += 1;
        mvprintw(queue_line,0,&format!("{:>5} {:>7.*}     {}", story_pos, 2, s.revenue, s.headline));
    }


}   

fn update_screen(state: State) {
    draw_budget(state.budget);
    draw_stories(state.story_queue);
    draw_time(state.seconds_remaining);
}
