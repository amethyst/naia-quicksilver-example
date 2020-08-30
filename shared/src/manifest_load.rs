
use naia_shared::Manifest;

use crate::{AuthEvent, ExampleEntity, ExampleEvent, PointEntity, KeyCommand};

pub fn manifest_load() -> Manifest<ExampleEvent, ExampleEntity> {
    let mut manifest = Manifest::<ExampleEvent, ExampleEntity>::new();

    manifest.register_event(AuthEvent::get_builder());
    manifest.register_pawn(PointEntity::get_builder(), KeyCommand::get_builder());

    manifest
}
