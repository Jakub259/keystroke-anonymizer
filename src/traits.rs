use std::time::SystemTime;

use evdev::InputEvent;

pub trait Scheduler {
    fn schedule_event(
        &mut self,
        prev_event_time: SystemTime,
        current_event: InputEvent,
    ) -> SystemTime;
}
