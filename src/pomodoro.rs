use std::{collections::HashMap, thread, time::Duration};

use crate::notification::Notification;

struct State {
    duration: Duration,
    notify: Notification,
    next_state: States,
}

impl State {
    fn count(&self) {
        println!("Count: {} minutes", self.duration.as_secs() / 60);
        thread::sleep(self.duration);
    }
}

#[derive(PartialEq, Eq, Hash)]
enum States {
    ShortBreak,
    Work,
}

pub struct Pomodoro {
    states: HashMap<States, State>,
}

impl Pomodoro {
    pub fn new(short_break_time: u64, work_time: u64) -> Self {
        Pomodoro {
            states: HashMap::from([
                (
                    States::Work,
                    State {
                        duration: Duration::from_secs(60 * work_time),
                        notify: Notification::new(
                            "Go to work :))".to_string(),
                            format!("Go to work for {} minutes", work_time),
                        ),
                        next_state: States::ShortBreak,
                    },
                ),
                (
                    States::ShortBreak,
                    State {
                        duration: (Duration::from_secs(60 * short_break_time)),
                        notify: Notification::new(
                            "Short break".to_string(),
                            format!("Take a short break for {} minutes", short_break_time),
                        ),
                        next_state: States::Work,
                    },
                ),
            ]),
        }
    }

    pub fn run(&self) {
        let mut state = &States::Work;
        loop {
            let state_impl = &self.states[state];
            let result = state_impl.notify.show();
            match result {
                Ok(_) => {},
                Err(e) => println!("{}", e)
            }
            state_impl.count();
            state = &state_impl.next_state;
        }
    }
}
