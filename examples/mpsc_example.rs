use fbp_pipe_test_rs::forwarder::mpsc_forwarder;
use fbp_pipe_test_rs::pipe::{RecvPipe, SendPipe};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct Component {
    on_publish: Box<dyn FnMut(String)>,
    on_consume: Box<dyn FnMut() -> Option<String>>,
    name: String,
    state: String,
    duration: Duration,
}

impl Component {
    pub fn run(mut self) {
        loop {
            println!("{}: Publish ({})", &self.name, &self.state);
            (self.on_publish)(format!("{} -> {}", self.name, self.state));
            if let Some(new_state) = (self.on_consume)() {
                self.state = new_state;
                println!("{}: Received ({})", &self.name, &self.state);
            }
            sleep(self.duration);
        }
    }
}

fn main() {
    let (mut sub2obj_in, mut sub2obj_out) = mpsc_forwarder::forwarder();
    let (mut obj2sub_in, mut obj2sub_out) = mpsc_forwarder::forwarder();

    let t2 = {
        thread::spawn(move || {
            let subject = Component {
                on_publish: Box::new(move |arg| sub2obj_in.send(arg)),
                on_consume: Box::new(move || obj2sub_out.recv()),
                name: "Subject".to_string(),
                state: "s".to_string(),
                duration: Duration::from_secs(1),
            };
            subject.run();
        })
    };

    let t1 = thread::spawn(move || {
        let object = Component {
            on_publish: Box::new(move |arg| obj2sub_in.send(arg)),
            on_consume: Box::new(move || sub2obj_out.recv()),
            name: "Object".to_string(),
            state: "o".to_string(),
            duration: Duration::from_secs(3),
        };
        object.run()
    });

    if let Err(e) = t1.join() {
        eprintln!("Error on t1 join: {:?}", e)
    }
    if let Err(e) = t2.join() {
        eprintln!("Error on t2 join: {:?}", e)
    }
}
