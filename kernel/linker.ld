/* Utopia OS Kernel Linker Script */
/* 支持多种启动协议 */

/* 默认入口点 */
ENTRY(_start)

SECTIONS {
    /* 内核加载地址 */
    . = 0x100000;

    /* Stivale2 头必须在文件的前 64KB 内，并且 16 字节对齐 */
    .stivale2hdr ALIGN(16) : {
        KEEP(*(.stivale2hdr))
    }

    /* Multiboot 2 头必须在文件的前 32768 字节内 */
    .multiboot2 ALIGN(8) : {
        KEEP(*(.multiboot2))
    }

    /* 代码段 */
    .text ALIGN(4K) : {
        *(.text .text.*)
    }

    /* 只读数据段 */
    .rodata ALIGN(4K) : {
        *(.rodata .rodata.*)
    }

    /* 数据段 */
    .data ALIGN(4K) : {
        *(.data .data.*)
    }

    /* BSS 段 */
    .bss ALIGN(4K) : {
        *(COMMON)
        *(.bss .bss.*)
    }

    /* 丢弃不需要的段 */
    /DISCARD/ : {
        *(.comment)
        *(.note)
        *(.note.*)
    }
}
