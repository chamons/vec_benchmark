const LARGE_SIZE: usize = 200;

#[derive(Debug)]
pub enum Event {
    Large([i32; LARGE_SIZE]),
    Small(String),
}

pub fn function_that_returns_vec() -> Vec<Event> {
    let mut events = vec![Event::Large([0; LARGE_SIZE])];
    events.push(Event::Small("asdf".to_string()));
    events
}

pub fn function_that_returns_single_vec() -> Vec<Event> {
    let events = vec![Event::Large([0; LARGE_SIZE])];
    events
}

#[derive(Debug)]
pub enum Events {
    One(Event),
    Many(Vec<Event>),
}

impl Events {
    pub fn new(event: Event) -> Self {
        Events::One(event)
    }
    pub fn push(&mut self, event: Event) {
        let prev = std::mem::replace(self, Events::Many(Vec::new()));
        let mut values = match prev {
            Events::One(e) => vec![e],
            Events::Many(v) => v,
        };
        values.push(event);
        *self = Events::Many(values);
    }
}

pub fn function_that_returns_enum() -> Events {
    let mut events = Events::new(Event::Large([0; LARGE_SIZE]));
    events.push(Event::Small("asdf".to_string()));
    events
}

pub fn function_that_returns_only_single_enum() -> Events {
    Events::new(Event::Large([0; LARGE_SIZE]))
}
