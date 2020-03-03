#[derive(Debug)]
pub struct State {
    pub show_hidden: bool,
}

impl State {
    pub fn new(show_hidden: bool) -> State {
        State { show_hidden }
    }
}
