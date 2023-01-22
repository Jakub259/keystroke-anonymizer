use std::time::Duration;
use std::time::SystemTime;

use evdev::InputEvent;

use crate::traits::Scheduler;

//buffer events for time equal to delay and then send them all at once
pub struct AtConstantTimeScheduler {
    delay: Duration,
}

impl AtConstantTimeScheduler {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }
}

impl Scheduler for AtConstantTimeScheduler {
    fn schedule_event(&mut self, _prev_time: SystemTime, current_event: InputEvent) -> SystemTime {
        let current_event_time = current_event.timestamp();

        let current_event_time_milis = current_event_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis();
        let delay = missing_to_next_divisible_by(current_event_time_milis, self.delay.as_millis());

        current_event_time + Duration::from_millis(delay as u64)
    }
}

fn missing_to_next_divisible_by(number: u128, divisor: u128) -> u128 {
    let remainder = number % divisor;
    if remainder == 0 {
        0
    } else {
        divisor - remainder
    }
}
