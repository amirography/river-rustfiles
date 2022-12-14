use std::sync::Arc;

use super::ctl;
use crate::config::Config;

pub async fn inputs(conf: Arc<Config>) {
    const INPUT: &str = "input";
    const ENABLED: &str = "enabled";

    let que: Vec<Vec<&str>> = vec![
        vec![INPUT, &conf.touch, "drag", ENABLED],
        vec![INPUT, &conf.touch, "tap", ENABLED],
        vec![INPUT, &conf.touch, "events", ENABLED],
        vec![INPUT, &conf.touch, "natural-scroll", ENABLED],
        vec![INPUT, &conf.touch, "scroll-method", "two-finger"],
    ];

    let mut handles = vec![];
    for command in que.iter() {
        handles.push(ctl(command.to_vec()));
    }
    for handle in handles {
        handle.await;
    }
}
