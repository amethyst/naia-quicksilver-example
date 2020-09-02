
use log::info;

use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{PointEntity, KeyCommand};

const SQUARE_SPEED: u16 = 10;

pub fn process_command(key_command: &KeyCommand, point_entity: &Rc<RefCell<PointEntity>>) {
    if *key_command.w.get() || *key_command.s.get() || *key_command.a.get() || *key_command.d.get() {
        info!("command received");
    } else {
        info!("empty command received");
    }

    let old_x: u16;
    let old_y: u16;
    {
        let entity_ref = point_entity.borrow();
        old_x = *(entity_ref.x.get());
        old_y = *(entity_ref.y.get());
    }
    if *key_command.w.get() {
        point_entity.borrow_mut().y.set(old_y.wrapping_sub(SQUARE_SPEED))
    }
    if *key_command.s.get() {
        point_entity.borrow_mut().y.set(old_y.wrapping_add(SQUARE_SPEED))
    }
    if *key_command.a.get() {
        point_entity.borrow_mut().x.set(old_x.wrapping_sub(SQUARE_SPEED))
    }
    if *key_command.d.get() {
        point_entity.borrow_mut().x.set(old_x.wrapping_add(SQUARE_SPEED))
    }
}