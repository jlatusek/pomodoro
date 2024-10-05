use std::{thread, time::Duration};

use crate::{icon::Tray, notification::Notification};

pub struct Pomodoro {
    short_break_time: Duration,
    work_time: Duration,
    short_break_notify: Notification,
    work_time_notify: Notification,
}

impl Pomodoro {
    pub fn new(short_break_minutes: u64, work_time_minutes: u64) -> Self {
        Pomodoro {
            short_break_time: Duration::from_secs(60 * short_break_minutes),
            work_time: Duration::from_secs(60 * work_time_minutes),
            short_break_notify: Notification::new(
                format!("Take a short break for {} minutes", short_break_minutes),
                "Short break".to_string(),
            ),
            work_time_notify: Notification::new(
                format!("Go to work for {} minutes", work_time_minutes),
                "Go to work :))".to_string(),
            ),
        }
    }

    pub fn run(&self) {
        let tray = Tray::new();
        loop {
            self.do_short_break();
            self.do_work();
        }
    }

    pub fn do_short_break(&self) {
        println!(
            "Short break make a nap for {} minutes",
            self.short_break_time.as_secs() / 60
        );
        self.short_break_notify.notify();
        thread::sleep(self.short_break_time);
    }

    pub fn do_work(&self) {
        println!("Go to work for {} minutes", self.work_time.as_secs() / 60);
        self.work_time_notify.notify();
        thread::sleep(self.work_time);
    }
}
