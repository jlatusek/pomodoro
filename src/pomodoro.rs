use rust_fsm::*;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use crate::notification::{Notification, NotifyStatus};

state_machine! {
    #[derive(Debug)]
    #[repr(C)]
    pomodoro(Idle)

    Idle(StartWork) => Work,
    Work => {
        Continue => ExtendWork,
        Skip => Skipped,
        StartLongBreak => LongBreak,
        StartShortBreak => ShortBreak,
    },
    ShortBreak => {
        Continue => ExtendShortBreak,
        Skip => Skipped,
        StartWork => Work,
    },
    LongBreak => {
        Continue => ExtendLongBreak,
        Skip => Skipped,
        StartWork => Work,
    },
    ExtendWork => {
        Continue => ExtendWork,
        StartLongBreak => LongBreak,
        StartShortBreak => ShortBreak,
    },
    ExtendShortBreak => {
        Continue => ExtendShortBreak,
        StartWork => Work,
    },
    ExtendLongBreak => {
        Continue => ExtendLongBreak,
        StartWork => Work,
    }
}

struct State {
    duration: Duration,
    notify: Notification,
}

impl State {
    async fn count(&self) {
        println!("Count: {} minutes", self.duration.as_secs() / 60);
        sleep(self.duration).await;
    }
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

    pub async fn run(&self) {
        let mut state = &States::Work;
        loop {
            let state_impl = &self.states[state];
            let result = state_impl.notify.show().await;
            let mut skip_count: bool = true;
            match result {
                Ok(result) => {
                    skip_count = match result {
                        NotifyStatus::AcceptStateChange => false,
                        NotifyStatus::SkipChange => true,
                    }
                }
                Err(e) => println!("{}", e),
            }
            if !skip_count {
                state_impl.count().await;
            }
            state = &state_impl.next_state;
        }
    }
}
