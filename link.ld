
MEMORY
{
    rom (rx)     : ORIGIN = 0x00000000,   LENGTH = 0x00180000
    stack (rw)   : ORIGIN = 0x08000000,   LENGTH = 0x00001500
    ram (rwx)    : ORIGIN = 0x08001500,   LENGTH = 0x0003EB00
    vimram (rx)  : ORIGIN = 0xFFF82000,   LENGTH = 0x000004A0
}

MPU_MIN_ALIGN = 8K;

__stack_size__ = 0x00001500;
__heap_size__  = HEAP_LENGTH;

/* Stack pointers in all CPU modes */
USER_SP  = 0x08001000;
SVC_SP   = 0x08001100;
FIQ_SP   = 0x08001200;
IRQ_SP   = 0x08001300;
ABORT_SP = 0x08001400;
UNDEF_SP = 0x08001500;

PROVIDE(__user_sp = USER_SP);
PROVIDE(__svc_sp = SVC_SP);
PROVIDE(__fiq_sp = FIQ_SP);
PROVIDE(__irq_sp = IRQ_SP);
PROVIDE(__abort_sp = ABORT_SP);
PROVIDE(__undef_sp = UNDEF_SP);

SECTIONS
{
    /DISCARD/ : { *(.note.gnu.*) }

    /* ARM Cortex-R4 Vector Table */
    .intvecs :
    {
        . = ALIGN(4);
        KEEP(*(.intvecs))
    } > rom

    .text :
    {
        . = ALIGN(4);
        _stext = .;
      
        *(.text .text.*)
        *(.rodata .rodata* 

    } > rom

    . = ALIGN(4);
    _etext = .;

    .data :
    {
        _sidata = LOADADDR(.data);
        . = ALIGN(4);
        _sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        _edata = .;
    } > ram AT > rom

    .stack (NOLOAD) :
    {
        . = ALIGN(8);
        _sstack = .;
        . = . + __stack_size__;
        . = ALIGN(8);
        _estack = .;
    } > stack

    .sram (NOLOAD) :
    {
        /* BSS section. */
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
        . = ALIGN(MPU_MIN_ALIGN);
    } > ram

    .heap :
    {
        _sheap = .;
        . = . + __heap_size__;
        _eheap = .;
    } > ram

    .vim (NOLOAD) :
    {
        . = ALIGN(4);
        KEEP(*(.vim_table .vim_table.*))
        . = ALIGN(4);
    } > vimram
}

PROVIDE(_heap_start = _sheap);
PROVIDE(_heapsize = __heap_size__);
PROVIDE(_ram_start = ORIGIN(ram));
PROVIDE(_ram_end = ORIGIN(ram) + LENGTH(ram));
