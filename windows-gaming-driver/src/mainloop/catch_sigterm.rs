use std::os::unix::io::{AsRawFd, RawFd};
use nix::sys::signalfd::{SigSet, SignalFd, SFD_CLOEXEC};
use nix::sys::signal;

use mainloop::*;

pub struct CatchSigterm {
    sigfd: SignalFd,
    monitor: MonitorRef,
}

impl CatchSigterm {
    pub fn new(monitor: MonitorRef) -> CatchSigterm {
        let mut sigset = SigSet::empty();
        sigset.add(signal::SIGTERM);
        sigset.thread_block().unwrap();
        CatchSigterm {
            sigfd: SignalFd::with_flags(&sigset, SFD_CLOEXEC).expect("Failed to create signalfd"),
            monitor: monitor,
        }
    }
}

impl Pollable for CatchSigterm {
    fn fd(&self) -> RawFd {
        self.sigfd.as_raw_fd()
    }

    fn run(&mut self) -> PollableResult {
        self.sigfd.read_signal().expect("Failed to read signalfd").unwrap();

        // sigterm -> shutdown
        self.monitor.borrow_mut().shutdown();

        PollableResult::Ok
    }
}