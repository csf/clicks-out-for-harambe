#[derive(Clone)]
pub struct State {
   pub seconds_remaining: u64,
   pub budget: f64,
   pub story_queue: Vec<Story>,
}

#[derive(Clone)]
pub struct Story {
    pub headline: String,
    words: u64,
    pub revenue: f64,
}

impl State {
    pub fn new() -> State {
        State { seconds_remaining: 1440, budget: 10_000.0, story_queue: Vec::new(), }
    }

    pub fn tick(&mut self) {
        if self.seconds_remaining > 1 {
            self.seconds_remaining -= 1;
        } else {
            // eventually notify main loop here.
            self.seconds_remaining = 0;
        }
    }

    pub fn publish(&mut self, story: Story) {
        self.story_queue.push(story);
    }
}

impl Story {
    pub fn new(headline: String, word_count: u64) -> Story {
        Story { headline: headline, words: word_count, revenue: 0.0 }
    }
}
