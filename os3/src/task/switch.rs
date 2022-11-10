use super::context::TaskContext;

core::arch::global_asm!(include_str!("switch.S"));

extern "C" {
    // 修改当前task上下文，下一个只需要读即可
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
