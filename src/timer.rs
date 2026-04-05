use std::{io::Write, time::Instant};

const INSPECTION_TIME: f64 = 15.0;

#[derive(Default)]
enum TimerState {
    #[default]
    Idle,
    InspectionCountdown(Instant),
    Solving(Instant)
}

#[derive(Default)]
pub struct Timer {
    state: TimerState
}

impl Timer {

    pub fn handle_keypress(&mut self) {
        match self.state {
            TimerState::Idle => {
                self.state = TimerState::InspectionCountdown(Instant::now());
            },
            TimerState::InspectionCountdown(_) => {
                self.state = TimerState::Solving(Instant::now());
            },
            TimerState::Solving(start) => {
                println!("you solved in {:.2} s", start.elapsed().as_secs_f64());
                self.state = TimerState::Idle;
            },
        }
    }

    fn get_timer_reading(&self) -> f64 {
        match self.state {
            TimerState::Idle => 0.0,
            TimerState::InspectionCountdown(start) => INSPECTION_TIME - start.elapsed().as_secs_f64(),
            TimerState::Solving(start) => start.elapsed().as_secs_f64()
        }
    }

    pub fn tick(&mut self) {
        print!("{:.2}      \r", self.get_timer_reading());
        std::io::stdout().flush().unwrap();
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
    }

}