use std::time::SystemTime;

pub struct State {
    pub start_time: SystemTime,
}

impl Default for State {
    fn default() -> Self {
        State {
            start_time: SystemTime::now(),
        }
    }
}
