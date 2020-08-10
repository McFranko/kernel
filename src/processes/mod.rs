#![allow(non_snake_case)]

pub struct Thread {
    stackPointer: u64,
    basePointer: u64,
    destinationIndex: u64,
}

impl Thread {
    pub unsafe fn switch(&self) {
        asm!("  
            mov RSP, {0}
            mov RBP, {1}
            mov RDI, {2}
        ",  in(reg) self.stackPointer, in(reg) self.basePointer,
            in(reg) self.destinationIndex)
    }
}
