use std::process::Command;

pub struct Notification {
    message: String,
    title: String,
}

impl Notification {
    pub fn new<S: Into<String>>(message: S, title: S) -> Self {
        Self {
            message: message.into(),
            title: title.into(),
        }
    }
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
