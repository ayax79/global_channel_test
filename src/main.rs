use crossbeam_channel::{unbounded, Sender, Receiver};
use std::sync::Arc;
use lazy_static::lazy_static;
use std::os::raw::c_void;

lazy_static! {
    static ref CHANNEL: Arc<ChannelHolder> = build_channel();
}

fn build_channel() -> Arc<ChannelHolder> {
    Arc::new(ChannelHolder::new())
}

struct ChannelHolder {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl ChannelHolder {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        ChannelHolder {
            sender,
            receiver,
        }
    }
}

fn main() {
    println!("Hello, world!");
}


extern "C" fn function_for_callback(_: *mut c_void) {
    Arc::clone(&CHANNEL).sender.send("foo".to_owned());
}
