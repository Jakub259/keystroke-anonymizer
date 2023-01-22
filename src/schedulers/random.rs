use std::time::Duration;
use std::time::SystemTime;

use evdev::InputEvent;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::traits::Scheduler;

//aply random delay smaller than max_delay to all events
pub struct RandomScheduler {
    max_delay: Duration,
    pub rng: ThreadRng,
}

impl RandomScheduler {
    pub fn new(max_delay: Duration) -> Self {
        Self {
            max_delay,
            rng: rand::thread_rng(),
        }
    }
}

impl Scheduler for RandomScheduler {
    fn schedule_event(
        &mut self,
        prev_event_time: SystemTime,
        current_event: InputEvent,
    ) -> SystemTime {
        let lower_bound = prev_event_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_millis(0));

        let current_event_time = current_event
            .timestamp()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_millis(0));
        let upper_bound = current_event_time + self.max_delay;

        // gen_range panics if lower_bound >= upper_bound
        if lower_bound >= upper_bound {
            return prev_event_time;
        }

        let random_delay = self.rng.gen_range(lower_bound..upper_bound);
        SystemTime::UNIX_EPOCH + random_delay
    }
}
