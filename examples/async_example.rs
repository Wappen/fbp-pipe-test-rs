use fbp_pipe_test_rs::{SendPipe, SendPipeImpl};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{mem, thread};

struct Logger {
    state: Mutex<String>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            state: Mutex::new("initial".to_string()),
        }
    }
}

impl Logger {
    pub fn run(&self) {
        loop {
            println!("State: {}", self.state.lock().expect("Lock state"));
            sleep(Duration::from_secs(1))
        }
    }

    pub fn set_state(&self, new_state: String) {
        let mut state = self.state.lock().expect("Lock state for setting");
        let _ = mem::replace(&mut *state, new_state);
    }
}

struct Clock {
    on_tick: Box<dyn FnMut(String)>,
}

impl Clock {
    pub fn run(&mut self) {
        let start_time = Instant::now();
        loop {
            let elapsed = Instant::now() - start_time;
            (self.on_tick)(format!("elapsed secs: {}", elapsed.as_secs_f32()));
            sleep(Duration::from_millis(300))
        }
    }
}

fn main() {
    let logger = Arc::new(Logger::default());

    let mut logger_in = {
        let logger = logger.clone();

        SendPipeImpl {
            on_send: Box::new(move |str| logger.set_state(str)),
        }
    };

    let mut clock = Clock {
        on_tick: Box::new(move |input| logger_in.send(input)),
    };

    thread::spawn(move || logger.run());
    sleep(Duration::from_secs(2));
    clock.run();
}
