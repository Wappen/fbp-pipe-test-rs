use fbp_pipe_test_rs::pipe::{SendPipe, SendPipeImpl};
use fbp_pipe_test_rs::transformer::SyncTransformer;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Calculator {}

impl Calculator {
    pub fn calc(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct Printer {}

impl Printer {
    pub fn print(&self, val: i32) {
        println!("{val}");
    }
}

fn main() {
    let calculator = Calculator {};
    let printer = Printer {};

    let printer_in = SendPipeImpl {
        on_send: Box::new(move |val| {
            printer.print(val);
        }),
    };

    let mut calculator_transformer = SyncTransformer::new(
        Box::new(move |(a, b)| calculator.calc(a, b)),
        Box::new(printer_in),
    );

    let start_time = Instant::now();
    loop {
        let a = (Instant::now() - start_time).as_millis() as i32;
        let b = (Instant::now() - start_time).as_secs() as i32;
        calculator_transformer.send((a, b));
        sleep(Duration::from_secs(1));
    }
}
