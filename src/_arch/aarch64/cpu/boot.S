.section ".text._start"

.global _start

_start:
	mrs x1, mpidr_el1
	and x1, x1, #3
	cbz x1, core2
core1:
	wfe
	b core1
core2:
	ldr     x1, =_start     
	mov     sp, x1                   
	bl      runtime_init    
	b       core1              
