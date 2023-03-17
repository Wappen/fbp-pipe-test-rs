use fbp_pipe_test_rs::pipe::{SendPipe, SendPipeImpl};
use fbp_pipe_test_rs::transformer::SyncTransformer;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{mem, thread};

/// Logs it's state every second
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
            println!("log: {}", self.state.lock().expect("Lock state"));
            sleep(Duration::from_secs(1))
        }
    }

    pub fn set_state(&self, new_state: String) {
        let mut state = self.state.lock().expect("Lock state for setting");
        let _ = mem::replace(&mut *state, new_state);
    }
}

/// Ticks every 300ms
struct Clock {
    on_tick: Box<dyn FnMut(Duration)>,
}

impl Clock {
    pub fn run(&mut self) {
        let start_time = Instant::now();
        loop {
            let elapsed = Instant::now() - start_time;
            (self.on_tick)(elapsed);
            sleep(Duration::from_millis(300))
        }
    }
}

fn main() {
    let logger = Arc::new(Logger::default());

    let logger_in = {
        let logger = logger.clone();

        SendPipeImpl {
            on_send: Box::new(move |str| logger.set_state(str)),
        }
    };

    let mut time2str_transformer = SyncTransformer::new(
        Box::new(|d: Duration| format!("{:.2}s", d.as_secs_f32())),
        Box::new(logger_in),
    );

    let mut clock = Clock {
        on_tick: Box::new(move |input| time2str_transformer.send(input)),
    };

    thread::spawn(move || logger.run());
    sleep(Duration::from_secs(2));
    clock.run();
}
