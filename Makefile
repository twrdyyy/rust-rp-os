BSP ?= rpi3

ifeq ($(BSP),rpi3)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE = raspi3
    QEMU_RELEASE_ARGS = -d in_asm -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    LINKER_FILE       = src/bsp/raspberrypi/link.ld
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53
endif

export LINKER_FILE

RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings -D missing_docs

FEATURES      = bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
    --features $(FEATURES)         \
    --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

KERNEL_ELF = target/$(TARGET)/release/kernel

DOCKER_IMAGE         = rustembedded/osdev-utils
DOCKER_CMD           = docker run --rm -v $(shell pwd):/work/tutorial -w /work/tutorial
DOCKER_CMD_INTERACT  = $(DOCKER_CMD) -i -t

DOCKER_QEMU     = $(DOCKER_CMD_INTERACT) $(DOCKER_IMAGE)
DOCKER_ELFTOOLS = $(DOCKER_CMD) $(DOCKER_IMAGE)

EXEC_QEMU = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)

.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) doc qemu clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

doc:
	$(DOC_CMD) --document-private-items

ifeq ($(QEMU_MACHINE_TYPE),)
qemu:
	@echo "This board is not yet supported for QEMU."
else
qemu: $(KERNEL_BIN)
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)
endif

clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(KERNEL_BIN)

readelf: $(KERNEL_ELF)
	readelf --headers $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	@$(DOCKER_ELFTOOLS) $(OBJDUMP_BINARY) --disassemble --demangle $(KERNEL_ELF) | rustfilt

nm: $(KERNEL_ELF)
	@$(DOCKER_ELFTOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt

check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json