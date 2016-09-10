#[derive(Clone)]
pub struct State {
   pub seconds_remaining: u64,
   pub budget: f64,
   pub story_queue: Vec<Story>,
   pub draft_headline: String,
}

#[derive(Clone)]
pub struct Story {
    pub headline: String,
    words: u64,
    pub revenue: f64,
    pub clicks: u64,
}

impl State {
    pub fn new() -> State {
        State { seconds_remaining: 1440, budget: 10_000.0, story_queue: Vec::new(), draft_headline:
        String::new() }
    }

    pub fn tick(&mut self) {
        if self.seconds_remaining > 1 {
            self.seconds_remaining -= 1;

            let clicks_per_story = self.model_clicks();
            let clicks = clicks_per_story.iter().sum();

            self.burn_rate(clicks);

        } else {
            // eventually notify main loop here so end of game can be handled.
            self.seconds_remaining = 0;
        }
    }

    pub fn publish(&mut self, story: Story) {
        self.story_queue.push(story);
    }

    // this needs to be the all-in burn rate to pay salaries and operate the hardware
    fn burn_rate(&mut self, clicks: u64) {
        self.budget -= clicks as f64 * 0.001 + 0.50;
    }

    // Machine learning smarts go here--update the clicks per story based on the audience demographics
    fn model_clicks(&mut self) -> Vec<u64> {
        let mut story_clicks = Vec::new();
        for s in self.story_queue.iter_mut() {
            // fake the click calculation
            story_clicks.push(15);
            s.clicks += 15;
        }
        story_clicks
    }
}

impl Story {
    pub fn new(headline: String, word_count: u64) -> Story {
        Story { headline: headline, words: word_count, revenue: 0.0, clicks: 0, }
    }
}
