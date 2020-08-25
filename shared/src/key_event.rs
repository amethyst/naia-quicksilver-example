use crate::ExampleEvent;
use naia_derive::Event;
use naia_shared::{Event};

#[derive(Event, Clone)]
#[type_name = "ExampleEvent"]
pub struct KeyEvent {
}

impl KeyEvent {
    fn is_guaranteed() -> bool {
        false
    }

    pub fn new() -> KeyEvent {
        return KeyEvent::new_complete();
    }
}
