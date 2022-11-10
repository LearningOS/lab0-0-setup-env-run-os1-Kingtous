



#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    ra: usize,      //寄存器ra
    sp: usize,      //
    s: [usize; 12], //调用者保存的12个寄存器
}

impl TaskContext {
    pub fn new_zero() -> Self {
        TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_restore(kstack_ptr: usize) -> Self{
        extern "C" {
            pub fn __restore();
        }
        Self {
            ra: __restore as _,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
