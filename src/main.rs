use crate::pomodoro::Pomodoro;

mod tray;
mod notification;
mod pomodoro;

fn main() {
    let pomodoro = Pomodoro::new(5, 20);
    pomodoro.run();
}
