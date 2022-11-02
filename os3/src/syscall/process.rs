//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::syscall::SYSCALL_EXIT;
use crate::task::task::TaskStatus;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, log_sys_call, get_task_running_time, get_task_syscall_times};
use crate::timer::get_time_us;

use super::{SYSCALL_YIELD, SYSCALL_TASK_INFO, SYSCALL_GET_TIME};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    log_sys_call(SYSCALL_EXIT);
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    log_sys_call(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    log_sys_call(SYSCALL_GET_TIME);
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // TODO
    log_sys_call(SYSCALL_TASK_INFO);
    unsafe {
        (*ti).status = TaskStatus::Running;
        (*ti).time = get_task_running_time();
        let times = get_task_syscall_times();
        let len = times.len();
        for i in 0..len {
            (*ti).syscall_times[i] = times[i];
        }
    }
    0
}
