use std::process::Command;

pub enum NotifyStatus {
    AcceptStateChange,
}

pub struct Notification {
    message: String,
    title: String,
}

impl Notification {
    pub fn new<S: Into<String>>(title: S, message: S) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
        }
    }
    pub fn show(&self) -> Result<NotifyStatus, String> {
        let output = Command::new("notify-send")
            .args([
                self.title.as_str(),
                self.message.as_str(),
                "-a",
                "Pomodoro",
                "-A",
                "Ok",
                "-t",
                format!("{}", 3600 * 1000).as_str(),
                "-w",
            ])
            .output()
            .expect("Failed to run notify-send program :((")
            .stdout;
        let output: i32 = (output[0] - 48).into();
        match output {
            0 => Ok(NotifyStatus::AcceptStateChange),
            _ => Err(
                "It seems that we got different result from notify-send than was expected"
                    .to_string(),
            ),
        }
    }
}
