use tokio::process::Command;

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
    pub async fn show(&self) -> Result<NotifyStatus, String> {
        Command::new("notify-send")
            .args([
                self.title.as_str(),
                self.message.as_str(),
                "-a",
                "Pomodoro",
                "-t",
                format!("{}", 3600 * 1000).as_str(),
                "-w",
            ])
            .output()
            .await
            .expect("Failed to run notify-send program :((");
        Ok(NotifyStatus::AcceptStateChange)
    }
}
