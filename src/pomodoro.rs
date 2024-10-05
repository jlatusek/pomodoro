use std::{collections::HashMap, thread, time::Duration};

use crate::notification::{Notification, NotifyStatus};

struct State {
    duration: Duration,
    notify: Notification,
    next_states: Vec<States>,
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
    PostponeShortBreak,
    Work,
    PostponeWork,
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
                        next_states: vec![States::ShortBreak, States::PostponeShortBreak],
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
                        next_states: vec![States::Work, States::PostponeShortBreak],
                    },
                ),
                (
                    States::PostponeWork,
                    State {
                        duration: Duration::from_secs(60 * 5),
                        notify: Notification::new("Postpone Work", "Ok you need some more chill"),
                        next_states: vec![States::Work, States::PostponeWork],
                    },
                ),
                (
                    States::PostponeShortBreak,
                    State {
                        duration: Duration::from_secs(60 * 5),
                        notify: Notification::new("Postpone Work", "Ok you need finish something"),
                        next_states: vec![States::ShortBreak, States::PostponeShortBreak],
                    },
                ),
            ]),
        }
    }

    pub fn run(&self) {
        let mut state = &States::Work;
        let mut notify_status: NotifyStatus;
        loop {
            let state_impl = &self.states[state];
            state_impl.count();
            notify_status = state_impl.notify.show().expect("Notification went wrong");
            state = match notify_status {
                NotifyStatus::Accept => &state_impl.next_states[0],
                NotifyStatus::Postpone => &state_impl.next_states[1],
            };
        }
    }
}
