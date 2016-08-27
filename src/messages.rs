
pub enum MainLoopMsg {
    Quit,
}

pub enum DisplayMsg {
    Time(String),
    UpdateDisplay,
    MainIntro,
    Splash,
    EndThread,
}
