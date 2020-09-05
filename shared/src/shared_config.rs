use std::time::Duration;

use naia_shared::{LinkConditionerConfig, SharedConfig};

pub fn get_shared_config() -> SharedConfig {
    let tick_interval = Duration::from_millis(150);

    //let link_condition = None;
    let link_condition = Some(LinkConditionerConfig::poor_condition());
    return SharedConfig::new(tick_interval, link_condition);
}
