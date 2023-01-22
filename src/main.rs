use std::num::NonZeroU64;
use std::time::Duration;

use evdev::Device;

use keystroke::arg_parser::parse_args;
use keystroke::event_loops::*;
use keystroke::schedulers::*;

fn main() {
    let matches = parse_args().get_matches();
    let keyboard = matches.get_one::<String>("keyboard").unwrap();
    let delay = matches.get_one::<NonZeroU64>("delay").unwrap();
    let delay = Duration::from_millis(delay.get());

    std::thread::sleep(Duration::from_millis(500));
    let mut keyboard = Device::open(keyboard).unwrap();
    if let Err(e) = keyboard.grab() {
        panic!("Error: {e}")
    }

    match matches.get_one::<String>("scheduler").unwrap().as_str() {
        "Random" => main_loop(keyboard, &mut RandomScheduler::new(delay)),
        "MaybeRandom" => main_loop(
            keyboard,
            &mut MaybeDelayScheduler::new(RandomScheduler::new(delay), 0.5),
        ),
        "AtConstantTime" => main_loop(keyboard, &mut AtConstantTimeScheduler::new(delay)),
        "MaybeAtConstantTime" => main_loop(
            keyboard,
            &mut MaybeDelayScheduler::new(AtConstantTimeScheduler::new(delay), 0.5),
        ),
        "Random2" => main_loop(keyboard, &mut Random2Scheduler::new(delay)),
        "MaybeRandom2" => main_loop(
            keyboard,
            &mut MaybeDelayScheduler::new(Random2Scheduler::new(delay), 0.5),
        ),
        _ => panic!("Invalid option -s/--scheduler"),
    }
}
