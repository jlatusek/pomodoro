use std::process::Command;

pub enum NotifyStatus {
    AcceptStateChange,
    SkipChange,
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
                "-A",
                "Skip",
                "-t",
                format!("{}", 3600 * 1000).as_str(),
                "-w",
            ])
            .output()
            .expect("Failed to run notify-send program :((")
            .stdout;
        if output.is_empty() {
            return Err("It seems that response from notification app was empty".to_string());
        }
        let output: i32 = (output[0] - 48).into();
        match output {
            0 => Ok(NotifyStatus::AcceptStateChange),
            1 => Ok(NotifyStatus::SkipChange),
            _ => Err(
                "It seems that we got different result from notify-send than was expected"
                    .to_string(),
            ),
        }
    }
}
