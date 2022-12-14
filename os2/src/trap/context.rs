use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    // 32个寄存器, 32*8 bytes
    pub x: [usize; 32],
    // 区分用户态/内核态, 8bytes
    pub sstatus: Sstatus,
    // Trap完成后要执行的，下一条指令的地址, 8bytes
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
