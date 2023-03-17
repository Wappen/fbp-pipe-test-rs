use fbp_pipe_test_rs::forwarder::BufferedForwarder;
use fbp_pipe_test_rs::pipe::{RecvPipe, SendPipe};
use std::sync::{Arc, Mutex};
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
    let sub2obj_forward = Arc::new(Mutex::new(BufferedForwarder::default()));
    let obj2sub_forward = Arc::new(Mutex::new(BufferedForwarder::default()));

    let t2 = {
        let sub2obj_forward = sub2obj_forward.clone();
        let obj2sub_forward = obj2sub_forward.clone();

        thread::spawn(move || {
            let subject = Component {
                on_publish: Box::new(move |arg| sub2obj_forward.lock().unwrap().send(arg)),
                on_consume: Box::new(move || obj2sub_forward.lock().unwrap().recv()),
                name: "Subject".to_string(),
                state: "s".to_string(),
                duration: Duration::from_secs(1),
            };
            subject.run();
        })
    };

    let t1 = thread::spawn(move || {
        let object = Component {
            on_publish: Box::new(move |arg| obj2sub_forward.lock().unwrap().send(arg)),
            on_consume: Box::new(move || sub2obj_forward.lock().unwrap().recv()),
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
