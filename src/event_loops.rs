use std::collections::VecDeque;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::spawn;
use std::time::{Duration, SystemTime};

use evdev::uinput::VirtualDevice;
use evdev::uinput::VirtualDeviceBuilder;
use evdev::Device;
use evdev::{EventType, InputEvent};
#[cfg(debug_assertions)]
use evdev::InputEventKind;

use crate::traits::Scheduler;

type InputEventQueue = Arc<Mutex<VecDeque<InputEvent>>>;

fn capture_events(mut dev: Device, queue: InputEventQueue) {
    loop {
        let events = match dev.fetch_events() {
            Ok(ev) => ev,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        let mut queue = queue.lock().unwrap();
        queue.extend(events);
    }
}

pub fn main_loop(dev: Device, scheduler: &mut impl Scheduler) {
    let mut uinput_dev = match new_uinput(&dev) {
        Ok(dev) => dev,
        Err(e) => panic!("Error: {e}"),
    };
    let event_queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut scheduled_event_queue = VecDeque::new();

    let queue_clone = event_queue.clone();
    let _x = spawn(move || {
        capture_events(dev, queue_clone);
    });
    let mut prev_event_time = SystemTime::UNIX_EPOCH;

    loop {
        thread::sleep(Duration::from_millis(1));
        let mut incoming_event_queue = event_queue.lock().unwrap();
        if let Some(event) = incoming_event_queue.pop_front() {
            if event.event_type() == EventType::KEY && (event.value() == 1 || event.value() == 0) {
                let last_scheduled_time = scheduled_event_queue
                    .back()
                    .map(|(_, t)| *t)
                    .unwrap_or_else(SystemTime::now);

                let new_time = scheduler.schedule_event(last_scheduled_time, event);
                scheduled_event_queue.push_back((event, new_time));
            } else {
                //println!("no delay on events that are not key (up or down)");
                uinput_dev.emit(&[event]).unwrap();
            }
        }

        while let Some(event) = scheduled_event_queue.front_mut() {
            if event.1 < prev_event_time {
                #[cfg(debug_assertions)]
                println!(
                    "Warning reordered event times... fixing: current: {:?}, prev: {:?}",
                    event.1, prev_event_time
                );
                event.1 = prev_event_time;
            }
            prev_event_time = event.1;
            let now = SystemTime::now();
            if event.1 < now {
                let event = scheduled_event_queue
                    .pop_front()
                    .expect("already checked for None");

                #[cfg(debug_assertions)]
                if let InputEventKind::Key(key) = event.0.kind() {
                    println!(
                        "time: {:0>9?} emitting event: {:<13?} state: {:05} delay: {:09?}",
                        now,
                        key,
                        if event.0.value() == 1 { "down" } else { "up" },
                        miliseconds_between(now, event.0.timestamp())
                    );
                }
                uinput_dev.emit(&[event.0]).unwrap();
            } else {
                break;
            }
        }
    }
}

fn new_uinput(dev: &Device) -> Result<VirtualDevice, Error> {
    let keys = dev.supported_keys().ok_or_else(Error::last_os_error)?;
    let builder = VirtualDeviceBuilder::new().expect("virtual device builder failed");
    let uinput = builder.name("uinput1").with_keys(keys)?.build()?;
    Ok(uinput)
}
#[cfg(debug_assertions)]
fn miliseconds_between(a: SystemTime, b: SystemTime) -> Duration {
    a.duration_since(b)
        .unwrap_or_else(|_| Duration::from_millis(0))
}
