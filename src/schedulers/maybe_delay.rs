use std::time::SystemTime;

use evdev::InputEvent;
use rand::{rngs::ThreadRng, Rng};

use crate::traits::Scheduler;

// modify a Scheduler so that it applies (or not) a random delay
pub struct MaybeDelayScheduler<S: Scheduler> {
    scheduler: S,
    probablity: f32,
    rng: ThreadRng,
}

#[allow(clippy::needless_return)]
impl<S: Scheduler> MaybeDelayScheduler<S> {
    pub fn new(scheduler: S, probablity: f32) -> Self {
        assert!((0.0..=1.0).contains(&probablity));
        Self {
            scheduler,
            probablity,
            rng: rand::thread_rng(),
        }
    }
}

impl<S: Scheduler> Scheduler for MaybeDelayScheduler<S> {
    #[allow(clippy::needless_return)]
    fn schedule_event(
        &mut self,
        prev_event_time: SystemTime,
        current_event: InputEvent,
    ) -> SystemTime {
        let delay_chance = self.rng.gen_range(0.0..1.0);
        if delay_chance < self.probablity {
            #[cfg(debug_assertions)]
            println!("delaying event");
            return self
                .scheduler
                .schedule_event(prev_event_time, current_event);
        } else {
            println!("not delaying event");
            //return greater value to prevent event reordeing
            return prev_event_time.max(current_event.timestamp());
        }
    }
}
