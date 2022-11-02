use core::{ptr::addr_of_mut};


use lazy_static::{lazy_static};
use super::{context::TaskContext, switch::__switch};

use crate::{
    config::MAX_APP_NUM,
    loader::{get_num_app, init_app_cx},
    sync::UPSafeCell,
};



#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready,
    Running,
    Exited,
}

// TCB
// 保存执行状态以及上下文
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub cx: TaskContext,
}

#[derive(Clone)]
pub struct TaskManager {
    pub app_cnt: usize,
    pub inner: UPSafeCell<TaskManagerInner>,
}

unsafe impl Sync for TaskManager {}

#[derive(Copy, Clone)]
pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = init_task_manager();
}

impl TaskManager {
    // 寻找下一个Ready的Task
    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task_index() {
            let mut inner = self.inner.as_mut();
            let current = inner.current_task;

            inner.tasks[next].status = TaskStatus::Running;
            inner.current_task = next;

            let current_task_cx_ptr = addr_of_mut!(inner.tasks[current].cx);
            let next_task_cx_ptr = addr_of_mut!(inner.tasks[next].cx);
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All applications completed.");
        }
    }

    // 寻找下一个task
    pub fn find_next_task_index(&self) -> Option<usize> {
        let inner = self.inner.as_mut();
        let current = inner.current_task;
        (current + 1..current + self.app_cnt + 1)
            .map(|id| id % self.app_cnt) // map防止越界
            .find(|id| inner.tasks[*id].status == TaskStatus::Ready)
    }

    // 标记当前任务为就绪态
    pub fn mark_current_suspend(&self) {
        let mut inner = self.inner.as_mut();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Ready;
    }

    // 标记当前任务中止
    pub fn mark_current_exit(&self) {
        let mut inner = self.inner.as_mut();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Exited;
    }

    // 运行第一个任务
    pub fn run_first_task(&self) {
        let mut inner = self.inner.as_mut();
        let first_task = &mut inner.tasks[0];
        first_task.status = TaskStatus::Running;

        let first_task_cx_ptr = addr_of_mut!(first_task.cx);
        drop(inner);
        let mut dummy_task_cx = TaskContext::new_zero();
        unsafe {
            __switch(addr_of_mut!(dummy_task_cx), first_task_cx_ptr);
        }
        panic!("unreachable!");
    }
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
