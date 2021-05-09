.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

.macro ADR_ABS register, symbol
    movz    \register, #:abs_g2:\symbol
    movk    \register, #:abs_g1_nc:\symbol
    movk    \register, #:abs_g0_nc:\symbol
.endm

.equ _core_id_mask, 0b11

.section .text._start


_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID      // provided by bsp/__board_name__/cpu.rs
	cmp	x1, x2
	b.ne	2f

	ADR_REL x0, __binary_nonzero_start
	ADR_ABS x1, __binary_nonzero_start
	ADR_ABS x2, __binary_nonzero_end_exclusive


1:	ldr x3, [x0], #8
    str x3, [x1], #8
    cmp x1, x2
    b.lo    1b

    ADR_ABS x0, __boot_core_stack_end_exclusive
    mov sp, x0

    ADR_ABS  x1, _start_rust
    br  x1

2:  wfe
    b   2b

.size	_start, . - _start
.type	_start, function
.global	_start