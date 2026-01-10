use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::{consts::{SIGTERM, SIGINT}, iterator::Signals};

#[derive(Clone)]
pub struct ShutdownFlag(Arc<AtomicBool>);

impl ShutdownFlag {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    pub fn request(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    pub fn is_requested(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

pub fn install_signal_handler(shutdown: ShutdownFlag) {
    let mut signals = Signals::new([SIGTERM, SIGINT]).unwrap();

    std::thread::spawn(move || {
        for _ in signals.forever() {
            shutdown.request();
        }
    });
}
