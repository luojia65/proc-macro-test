MEMORY {
    /* 存储单元的地址，可以是虚拟地址 */
	VIRT_DRAM : ORIGIN = 0x80000000, LENGTH = 128M
}

PROVIDE(_stext = 0x80400000);

/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

SECTIONS
{
    /* .text 字段 */
    .text _stext : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
        _etext = .;
    } > VIRT_DRAM

    /* .rodata 字段 */
    .rodata : ALIGN(4K) {
        _srodata = .;
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        _erodata = .;
    } > VIRT_DRAM

    /* .data 字段 */
    .data : ALIGN(4K) { 
        _sidata = LOADADDR(.data);
        _sdata = .;
        /* Must be called __global_pointer$ for linker relaxations to work. */
        PROVIDE(__global_pointer$ = . + 0x800);
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        . = ALIGN(4K);
        _edata = .;
    } > VIRT_DRAM

    /* .bss 字段 */
    .bss (NOLOAD) : ALIGN(4K) {
        _sbss = .;
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
        . = ALIGN(4K);
        _ebss = .;
    } > VIRT_DRAM

    .frame (INFO) : ALIGN(4K) {
        _sframe = .;
        . = ALIGN(4K);
        _eframe = .;
    } > VIRT_DRAM

    /* fictitious region that represents the memory available for the stack */
    .stack (INFO) : ALIGN(4K) {
        _estack = .;
        . = ALIGN(4K);
        _sstack = .;
    } > VIRT_DRAM

    /* Discard .eh_frame, we are not doing unwind on panic so it is not needed */
    /DISCARD/ :
    {
        *(.eh_frame_hdr);
    }
}
