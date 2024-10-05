use std::{process::Command, thread, time::Duration};

#[derive(Debug)]
pub struct Pomodoro {
    short_break_time: Duration,
    work_time: Duration,
    short_break_notify: Notification,
    work_time_notify: Notification,
}

#[derive(Debug)]
struct Notification {
    message: String,
    title: String,
}

impl Pomodoro {
    pub fn new(short_break_minutes: u64, work_time_minutes: u64) -> Self {
        Pomodoro {
            short_break_time: Duration::from_secs(60 * short_break_minutes),
            work_time: Duration::from_secs(60 * work_time_minutes),
            short_break_notify: Notification {
                message: (format!("Take a short break for {} minutes", short_break_minutes)),
                title: ("Short break").to_string(),
            },
            work_time_notify: Notification {
                message: (format!("Go to work for {} minutes", work_time_minutes)),
                title: ("Go to work :))").to_string(),
            },
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

impl Notification {
    pub fn notify(&self) {
        let output = Command::new("notify-send")
            .args([
                self.title.as_str(),
                self.message.as_str(),
                "-A",
                "OK",
                "-t",
                format!("{}", 3600 * 1000).as_str(),
                "-w",
            ])
            .output()
            .expect("Failed to run notify-send program :((");
        println!("Notify status: {}", output.status);
    }
}
