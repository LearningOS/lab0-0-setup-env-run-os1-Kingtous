

use crate::{syscall::process::TaskInfo};

use self::task::TASK_MANAGER;

pub mod context;
pub mod switch;
pub mod task;

// 快捷函数
pub fn suspend_current_and_run_next() {
    TASK_MANAGER.mark_current_suspend();
    TASK_MANAGER.run_next_task();
}

// 快捷函数
pub fn exit_current_and_run_next() {
    TASK_MANAGER.mark_current_exit();
    TASK_MANAGER.run_next_task();
}

// 运行第一个task
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

// 统计系统调用
#[inline]
pub fn log_sys_call(call_id: usize) {
    TASK_MANAGER.log_sys_call(call_id);
}

#[inline]
pub fn get_task_info(info: *mut TaskInfo) -> isize {
    TASK_MANAGER.get_task_info(info)
}
