OUTPUT_ARCH("riscv")
ENTRY("_start")
BASE_ADDRESS = 0x80200000;

/*
 * 其中 . 表示当前地址，也就是链接器会从它指向的位置开始往下放置
 * 从输入的目标文件中收集来的段。我们可以对 . 进行赋值来调整接下来的段放在哪里，
 * 也可以创建一些全局符号赋值为 .
 * 
 */
SECTIONS {
  . = BASE_ADDRESS;
  skernel = .;

  stext = .;
  /*
   * 冒号前面表示最终生成的可执行文件的一个段的名字，花括号内按照放置顺序描述将所有输入目标文件的哪些段放在这个段中，
   * 每一行格式为   <ObjectFile>(SectionName)，表示目标文件 ObjectFile 的名为 SectionName
   * 的段需要被放进去。我们也可以使用通配符来书写 <ObjectFile> 和 <SectionName> 分别表示可能的输入目标文件和段名。
   * 最终的合并结果是，在最终可执行文件中各个常见的段 .text, .rodata .data, .bss
   * 从低地址到高地址按顺序放置，每个段里面都包括了所有输入目标文件的同名段，且每个段都有两个全局符号给出了它的开始和结束地址
   * .text 段的开始和结束地址分别是 stext 和 etext 
   */
  .text : {
     *(.text.entry)
     *(.text .text.*)
  }

  . = ALIGN(4K);
  etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
