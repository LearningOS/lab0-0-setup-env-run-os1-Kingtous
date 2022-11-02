.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm

.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

    .section .text
    .globl __switch
    # 切换上下文
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
__switch:
    # 第一步：保存 current_task_cx_ptr
    # #[repr(C)]
    # pub struct TaskContext {
    #     ra: usize, //寄存器ra
    #     sp: usize, // 
    #     s: [usize; 12] //调用者保存的12个寄存器 
    # }
    sd sp, 8(a0) # 保存内核栈地址，a0的偏移+8
    sd ra, 0(a) #保存ra寄存器的值到a0+偏移0
    # 保存s0-s11寄存器
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    # 第二步：恢复 next_task_cx_ptr
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    # 恢复内核栈到next_task_cx_ptr
    ld sp, 8(a1)
    ret

