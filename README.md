# clicks-out-for-harambe
*Clickbait. Fame. Fortune.*

My first attempt at the [Ludum Dare](http://ludumdare.com/) Game Jam competition (#36). The theme: Ancient
Technology, which is why this is a terminal-based game. That, and because I didn't have time to start from
scratch learning graphics libraries and game engines for Rust.

## The Game
The year is 2016. During the depths of the credit crisis, you took the only job you could find, an unpaid
internship with PooSprinkler, one of the earliest clickbait sites. After years of toil trying to come up
with fresh content and listicles, you worked your way up to editor-in-chief. With only a small budget for
stories, you're reduced to begging others to create content for free.

But you have one superpower: the ability to write clickbait headlines which keep the stream of ad revenue
trickling in just enough to keep the lights on.

And then it happens: Harambe, beloved gorilla, is assassinated.

This is your time to turn your media empire around.

Spend your story budget wisely. Write great headlines. Know your audience. Get clicks, get paid.

A rise goes up from the internet: Clicks out for Harambe!

## Technology
* [Rust](https://rust-lang.org/)
* [ncurses-rs](https://github.com/jeaye/ncurses-rs)
* Currently only builds for Mac OS X.

## Build and Run
To build and run:

```
git clone https://github.com/csf/clicks-out-for-harambe
cd clicks-out-for-harambe
cargo build --release
./target/release/clicks-out-for-harambe
```

To build and run with the intro text skipped, use:

```
cargo build --release --features=skip-intro
```  

