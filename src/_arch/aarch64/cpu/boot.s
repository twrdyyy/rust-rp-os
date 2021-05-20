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
	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID      // provided by bsp/__board_name__/cpu.rs
	cmp	x1, x2
	b.ne	2f

	// If execution reaches here, it is the boot core.

	// Next, relocate the binary.
	ADR_REL	x0, __binary_nonzero_start         // The address the binary got loaded to.
	ADR_ABS	x1, __binary_nonzero_start         // The address the binary was linked to.
	ADR_ABS	x2, __binary_nonzero_end_exclusive

1:	ldr	x3, [x0], #8
	str	x3, [x1], #8
	cmp	x1, x2
	b.lo	1b

	// Set the stack pointer.
	ADR_ABS	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// Jump to the relocated Rust code.
	ADR_ABS	x1, _start_rust
	br	x1

	// Infinitely wait for events (aka "park the core").
2:	wfe
	b	2b

.size	_start, . - _start
.type	_start, function
.global	_start