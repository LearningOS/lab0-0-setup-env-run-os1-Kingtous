use core::{ptr::addr_of_mut};


use alloc::vec::Vec;
use lazy_static::{lazy_static};
use super::{context::TaskContext, switch::__switch};

use crate::{
    config::{MAX_APP_NUM, MAX_SYSCALL_NUM},
    loader::{get_num_app, init_app_cx},
    sync::UPSafeCell, timer::get_time,
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
    
    pub sche_st: usize, // task运行时间
    pub syscall_times: [u32; MAX_SYSCALL_NUM]
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
            // println!("[KERNEL] switch program {} to {}", current, next);
            inner.tasks[next].status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = addr_of_mut!(inner.tasks[current].cx);
            let next_task_cx_ptr = addr_of_mut!(inner.tasks[next].cx);
            // 初始化时间
            let initial_sche_st = inner.tasks[next].sche_st;
            if initial_sche_st == 0 {
                inner.tasks[next].sche_st = get_time();
            }
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
        println!("[KERNEL] task {} exited", current);
        inner.tasks[current].status = TaskStatus::Exited;
    }

    // 运行第一个任务
    pub fn run_first_task(&self) {
        let mut inner = self.inner.as_mut();
        let first_task = &mut inner.tasks[0];
        first_task.status = TaskStatus::Running;
        first_task.sche_st = get_time();

        let first_task_cx_ptr = addr_of_mut!(first_task.cx);
        drop(inner);
        let mut dummy_task_cx = TaskContext::new_zero();
        unsafe {
            __switch(addr_of_mut!(dummy_task_cx), first_task_cx_ptr);
        }
        panic!("unreachable!");
    }

    pub fn log_sys_call(&self, call_id: usize) {
        let mut inner = self.inner.as_mut();
        let task_index = inner.current_task;
        inner.tasks[task_index].syscall_times[call_id] += 1;
    }

    pub fn get_task_syscall_times(& self) -> Vec<u32> {
        let inner = self.inner.as_mut();
        Vec::from(inner.tasks[inner.current_task].syscall_times)
    }

    pub fn get_task_running_time(&self) -> usize {
        let inner = self.inner.as_mut();
        get_time() - inner.tasks[inner.current_task].sche_st
    }
}

fn init_task_manager() -> TaskManager {
    let app_cnt = get_num_app();
    let mut tasks = [TaskControlBlock {
        status: TaskStatus::UnInit,
        cx: TaskContext::new_zero(),
        sche_st: 0,
        syscall_times: [0; MAX_SYSCALL_NUM],
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
