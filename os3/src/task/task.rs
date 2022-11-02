use core::char::MAX;

use alloc::str;
use lazy_static::lazy_static;

use crate::{
    config::MAX_APP_NUM,
    loader::{get_num_app, init_app_cx},
    sync::UPSafeCell,
};

use super::context::TaskContext;

pub enum TaskStatus {
    UnInit, // 未初始化
    Ready,
    Running,
    Exited,
}

// TCB
// 保存执行状态以及上下文
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub cx: TaskContext,
}

pub struct TaskManager {
    pub app_cnt: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = init_task_manager();
}

fn init_task_manager() -> TaskManager {
    let app_cnt = get_num_app();
    let mut tasks = [TaskControlBlock {
        status: TaskStatus::UnInit,
        cx: TaskContext::new_zero(),
    }; MAX_APP_NUM];
    // 加载所有的task
    for (index, task) in tasks.iter_mut().enumerate().take(app_cnt) {
        task.cx = TaskContext::goto_restore(init_app_cx(index));
        task.status = TaskStatus::Ready;
    }
    TaskManager {
        app_cnt: app_cnt,
        inner: UPSafeCell::new(TaskManagerInner {
            tasks: tasks,
            current_task: 0,
        }),
    }
}
