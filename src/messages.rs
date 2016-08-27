
pub enum MainLoopMsg {
    Quit,
}

pub enum DisplayMsg {
    MainIntro,
    Tutorial,
    InitialScreen(u64),
}
