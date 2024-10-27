use crate::pomodoro::Pomodoro;
use clap::Parser;
use std::error::Error;
use tokio::io::unix::AsyncFd;

mod dbus;
mod notification;
mod pomodoro;

/// App to implement simple pomodoro timer with build in notification
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Minutes for short break
    #[arg(short, long, default_value_t = 5)]
    short_break_time: u64,

    /// Minutes to work hard
    #[arg(short, long, default_value_t = 30)]
    work_time: u64,
}

// fn main() {
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let handle = tokio::spawn(async move {
        let pomodoro = Pomodoro::new(args.short_break_time, args.work_time);
        pomodoro.run()
    });
    dbus::watch_screen_lock().await?;
    let _ = handle.await.unwrap();
    Ok(())
}
