use naia_derive::EventType;

use crate::{AuthEvent, KeyEvent};

#[derive(EventType, Clone)]
pub enum ExampleEvent {
    KeyEvent(KeyEvent),
    AuthEvent(AuthEvent),
}
