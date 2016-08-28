use gamestate::State;

pub enum MainLoopMsg {
    Quit,
}

pub enum DisplayMsg {
    MainIntro,
    Tutorial,
    InitialScreen(State),
    AnyKey,
    AnyKeyPause,
}
