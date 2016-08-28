pub struct State {
   pub seconds_remaining: u64,
   pub budget: f64,
   pub story_queue: Vec<Story>,
}

pub struct Story {
    pub headline: String,
    words: u64,
    pub revenue: f64,
}

impl State {
    pub fn new() -> State {
        State { seconds_remaining: 2400, budget: 10_000.0, story_queue: Vec::new(), }
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
