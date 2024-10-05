use crate::pomodoro::Pomodoro;
use clap::Parser;

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

fn main() {
    let args = Args::parse();
    let pomodoro = Pomodoro::new(args.short_break_time, args.work_time);
    pomodoro.run();
}
