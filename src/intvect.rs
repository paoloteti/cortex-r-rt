///
/// ARM Cortex-R Vectors Table
///

extern "C" {
    pub fn _cpu_reset();
    pub fn _undef_entry();
    pub fn _prefetch_abort();
    pub fn _phantom_interrupt();
    pub fn _svc_handler();
    pub fn _dabort();
}

// 1. Legacy ARM7 Mode: In this mode the software designer has
//    to write a software dispatcher for the FIQ and IRQ.
//
// 2. Vectored interrupt: In this mode the VIM module dose the
//    dispatching for IRQ and FIQ and has an register which shows
//    the address of the pending interrupts routine. The CPU has to
//    load the vector address to the PC via a LDR instruction.
//
// 3. Hardware Vectored Interrupt: This mode is only available for IRQ.
//    In this mode the VIM dose the dispatching and supplies the vector
//    address to the Cortex-RxF CPU via the VIC Port.
//    This mode is similar to #2 but it saves the load instruction on
//    address 0x18 and thus a few cycles.
//
// The "LDR PC, [PC, #-0x1B0]" instruction is necessary for #2.
// In this mode the CPU has to load the vector address from the VIM
// register IRQVECREG for IRQ's and FIQVECREG for FIQ's.
//
// IRQVECREG is at 0xFFFF_FE70
// FIQVECREG is at 0xFFFF_FE74
//
// "LDR PC, [PC, #-0x1B0]" at address 0x18 resolves to a load from address
// 0xFFFF_FE70. (-0x1B0 == 0xFFFF_FE50 --> 0xFFFF_FE50 + 0x18 + 0x8 = 0xFFFF_FE70)

global_asm!(
    r#"
    .section .intvecs,"a",%progbits
    .extern _cpu_reset
    .extern _undef_entry
    .extern _svc_handler
    .extern _prefetch_abort
    .extern _dabort
    .extern _phantom_interrupt
    .weak reset_entry

reset_entry:
    b   _cpu_reset
    b   _undef_entry
    b   _svc_handler
    b   _prefetch_abort
    b   _dabort
    b   _phantom_interrupt
    ldr pc,[pc,#-0x1b0]
    ldr pc,[pc,#-0x1b0]
"#
);
