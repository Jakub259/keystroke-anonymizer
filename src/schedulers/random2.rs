use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

use evdev::InputEvent;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::traits::Scheduler;

pub struct Random2Scheduler {
    max_delay: Duration,
    rng: ThreadRng,
    map: HashMap<u16, Duration>,
}

impl Random2Scheduler {
    pub fn new(max_delay: Duration) -> Self {
        Self {
            max_delay,
            rng: rand::thread_rng(),
            map: HashMap::new(),
        }
    }
}

impl Scheduler for Random2Scheduler {
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

        if current_event.code() == 425 && current_event.value() == 1 {
            #[cfg(debug_assertions)]
            println!("generated new identity");
            self.map.clear();
        }
        let upper_bound = current_event_time
            + *self
                .map
                .entry(current_event.code())
                .or_insert_with(|| self.rng.gen_range(Duration::from_millis(0)..self.max_delay));

        // gen_range panics if lower_bound >= upper_bound
        if lower_bound >= upper_bound {
            return prev_event_time;
        }

        #[cfg(debug_assertions)]
        println!("random delays map: {:?}", self.map);
        let random_delay = self.rng.gen_range(lower_bound..upper_bound);
        SystemTime::UNIX_EPOCH + random_delay
    }
}
