use std::process;
use std::ffi::CString;

use nix::sched;
use nix::sys::wait;
use nix::unistd::Pid;
use nix::sched::CloneFlags;
use nix::sys::wait::WaitPidFlag;

use crate::libcontainer::environment::Environment;

pub fn create(environment: &Environment) -> i32 {
    let stack = &mut[0; 1024*1024];
    let exec_fn = Box::new(|| child(&environment));

    match sched::clone(exec_fn, stack, CloneFlags::empty(), None) {
        Ok(pid) => pid.as_raw(),
        Err(err) => {
            eprintln!("clone error: {}", err);
            process::exit(-1);
        }
    }
}

pub fn wait(pid: i32) {
    match wait::waitpid(Pid::from_raw(pid), Some(WaitPidFlag::__WALL)) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("wait error: {}", err);
            process::exit(-1);
        }
    }
}

fn child(environment: &Environment) -> isize {
    return 0;
}
