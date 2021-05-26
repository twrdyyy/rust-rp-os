// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

// Load the address of a symbol into a register, absolute.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_ABS register, symbol
	movz	\register, #:abs_g2:\symbol
	movk	\register, #:abs_g1_nc:\symbol
	movk	\register, #:abs_g0_nc:\symbol
.endm
.equ _EL2, 0x8
.equ _core_id_mask, 0b11

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	mrs	x0, CurrentEL
	cmp	x0, _EL2
 	b.ne	1f
	
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID     
	cmp	x1, x2
	b.eq	1f

1:

	// Set the stack pointer.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0
	
    ldr x1,=_start_rust2
	
    mov x2,#0x40000000
	
    //str x1,[x2, #0xDC]
    sev
	b _start_rust
    #bx lr
	// Jump to the relocated Rust code.
	

// Infinitely wait for events (aka "park the core").
2:	wfe
	ADR_REL	x0, __boot_core2_stack_end_exclusive
	mov	sp, x0

	// Jump to the relocated Rust code.
	ADR_ABS	x1, _start_rust2
	br	x1

3:	wfe
	ADR_REL	x0, __boot_core3_stack_end_exclusive
	mov	sp, x0

	// Jump to the relocated Rust code.
	ADR_ABS	x1, _start_rust2
	br	x1

4:	wfe
	ADR_REL	x0, __boot_core4_stack_end_exclusive
	mov	sp, x0

	// Jump to the relocated Rust code.
	ADR_ABS	x1, _start_rust2
	br	x1

.size	_start, . - _start
.type	_start, function
.global	_start