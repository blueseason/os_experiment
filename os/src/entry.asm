	.section .text.entry   // 将第 2 行后面的内容全部放到一个名为 .text.entry 的中段
	.global _start         //  _start 是一个全局符号
_start:
	la sp, boot_stack_top
	call rust_main

	.section .bss.stack
	.global boot_stack_lower_bound
boot_stack_lower_bound:
	.space 4096 * 16
	.global	boot_stack_top
boot_stack_top:
	
