use fbp_pipe_test_rs::transformer::SyncTransformer;
use fbp_pipe_test_rs::*;
use std::thread::sleep;
use std::time::Duration;

struct Producer {}

impl Producer {
    pub fn calc(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct Consumer {}

impl Consumer {
    pub fn print(&self, val: i32) {
        println!("{val}");
    }
}

fn main() {
    let producer = Producer {};
    let consumer = Consumer {};

    let consumer_in = SendPipeImpl {
        on_send: Box::new(move |val| {
            consumer.print(val);
        }),
    };

    let mut produce_transformer = SyncTransformer::new(
        Box::new(move |(a, b)| producer.calc(a, b)),
        Box::new(consumer_in),
    );

    loop {
        produce_transformer.send((1, 5));
        sleep(Duration::from_secs(1));
    }
}
