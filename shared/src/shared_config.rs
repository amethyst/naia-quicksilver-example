use std::time::Duration;

use naia_shared::{LinkConditionerConfig, SharedConfig};

pub fn get_shared_config() -> SharedConfig {
    let tick_interval = Duration::from_millis(50);

    //let link_condition = None;
//    let link_condition = Some(LinkConditionerConfig::poor_condition());
    let link_condition = Some(LinkConditionerConfig::good_condition());
//    let link_condition = Some(LinkConditionerConfig {
//        incoming_latency: 1000,
//        incoming_jitter: 300,
//        incoming_loss: 0.01,
//        incoming_corruption: 0.00000001
//    });
    return SharedConfig::new(tick_interval, link_condition);
}
