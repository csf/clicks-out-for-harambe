
pub enum MainLoopMsg {
    Quit,
}

pub enum DisplayMsg {
    Time(String),
    UpdateDisplay,
    Splash,
    EndThread,
}
