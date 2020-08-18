use crate::{AuthEvent, ExampleEntity, ExampleEvent, PointEntity, StringEvent};
use naia_shared::Manifest;

pub fn manifest_load() -> Manifest<ExampleEvent, ExampleEntity> {
    let mut manifest = Manifest::<ExampleEvent, ExampleEntity>::new();

    manifest.register_event(AuthEvent::get_builder());
    manifest.register_event(StringEvent::get_builder());
    manifest.register_entity(PointEntity::get_builder());

    manifest
}
