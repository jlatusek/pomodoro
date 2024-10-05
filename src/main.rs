use crate::pomodoro::Pomodoro;

mod pomodoro;

fn main() {
    let pomodoro = Pomodoro::new(5, 20);
    loop {
        pomodoro.do_short_break();
        pomodoro.do_work();
    }
}
