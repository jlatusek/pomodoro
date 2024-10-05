use crate::pomodoro::Pomodoro;

mod icon;
mod notification;
mod pomodoro;

fn main() {
    let pomodoro = Pomodoro::new(5, 20);
    pomodoro.run();
}
