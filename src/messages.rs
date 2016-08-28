use gamestate::State;

pub enum MainLoopMsg {
    Tick,
    Quit,
}

pub enum DisplayMsg {
    MainIntro,
    Tutorial,
    InitialScreen(State),
    UpdateScreen(State),
    AnyKeyPause,
}
