use clap::value_parser;
use clap::{Arg, Command};
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy)]
pub enum Schedulers {
    Random,
    MaybeRandom,
    AtConstantTime,
    MaybeAtConstantTime,
    ConstantRandom,
    MaybeConstantRandom,
}

pub fn parse_args() -> Command {
    let delay = Arg::new("delay")
        .short('d')
        .long("delay")
        .help("Maximal delay that program may add to keypress")
        .value_parser(value_parser!(NonZeroU64))
        .required(true);
    let scheduler = Arg::new("scheduler")
        .short('s')
        .long("scheduler")
        .help("Scheduler that will be used to schedule keypresses")
        .required(true);
    let device = Arg::new("keyboard")
        .short('k')
        .long("keyboard")
        .help("Path to keyboard device, use command: \"s /dev/input/by-path | grep kbd\" to find available keyboards")
        .required(true);
    Command::new("keystroke")
        .arg(scheduler)
        .arg(device)
        .arg(delay)
}
