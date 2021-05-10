BSP ?= rpi3

# Default to a serial device name that is common in Linux.
DEV_SERIAL ?= /dev/ttyUSB0

# Query the host system's kernel name
UNAME_S = $(shell uname -s)

# BSP-specific arguments
ifeq ($(BSP),rpi3)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE = raspi3
    QEMU_RELEASE_ARGS = -serial stdio -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    READELF_BINARY    = aarch64-none-elf-readelf
    LINKER_FILE       = src/bsp/raspberrypi/link.ld
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53
    CHAINBOOT_DEMO_PAYLOAD = demo_payload_rpi3.img
else ifeq ($(BSP),rpi4)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE =
    QEMU_RELEASE_ARGS = -serial stdio -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    READELF_BINARY    = aarch64-none-elf-readelf
    LINKER_FILE       = src/bsp/raspberrypi/link.ld
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72
    CHAINBOOT_DEMO_PAYLOAD = demo_payload_rpi4.img
endif

# Export for build.rs
export LINKER_FILE

QEMU_MISSING_STRING = "This board is not yet supported for QEMU."

RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings -D missing_docs

FEATURES      = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
    $(FEATURES)                    \
    --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check --color always $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

KERNEL_ELF = target/$(TARGET)/release/kernel

DOCKER_IMAGE         = rustembedded/osdev-utils
DOCKER_CMD           = docker run --rm -v $(shell pwd):/work/tutorial -w /work/tutorial
DOCKER_CMD_INTERACT  = $(DOCKER_CMD) -i -t
DOCKER_ARG_DIR_UTILS = -v $(shell pwd)/../utils:/work/utils
DOCKER_ARG_DEV       = --privileged -v /dev:/dev

DOCKER_QEMU  = $(DOCKER_CMD_INTERACT) $(DOCKER_IMAGE)
DOCKER_TEST  = $(DOCKER_CMD) -t $(DOCKER_ARG_DIR_UTILS) $(DOCKER_IMAGE)
DOCKER_TOOLS = $(DOCKER_CMD) $(DOCKER_IMAGE)

# Dockerize commands that require USB device passthrough only on Linux
ifeq ($(UNAME_S),Linux)
    DOCKER_CMD_DEV = $(DOCKER_CMD_INTERACT) $(DOCKER_ARG_DEV)

    DOCKER_CHAINBOOT = $(DOCKER_CMD_DEV) $(DOCKER_ARG_DIR_UTILS) $(DOCKER_IMAGE)
endif

EXEC_QEMU     = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)
EXEC_MINIPUSH = ruby ../utils/minipush.rb
EXEC_QEMU_MINIPUSH = ruby tests/qemu_minipush.rb

.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) doc qemu qemuasm chainboot clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	$(call colorecho, "\nCompiling kernel - $(BSP)")
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

doc:
	$(call colorecho, "\nGenerating docs")
	@$(DOC_CMD) --document-private-items --open

ifeq ($(QEMU_MACHINE_TYPE),)
qemu test:
	$(call colorecho, "\n$(QEMU_MISSING_STRING)")
else
qemu: $(KERNEL_BIN)
	$(call colorecho, "\nLaunching QEMU")
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)
endif

qemuasm: $(KERNEL_BIN)
	$(call colorecho, "\nLaunching QEMU with ASM output")
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN) -d in_asm

test: $(KERNEL_BIN)
	$(call colorecho, "\nTesting chainloading - $(BSP)")
	@$(DOCKER_TEST) $(EXEC_QEMU_MINIPUSH) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) \
			-kernel $(KERNEL_BIN) $(CHAINBOOT_DEMO_PAYLOAD)

chainboot: $(KERNEL_BIN)
	@$(DOCKER_CHAINBOOT) $(EXEC_MINIPUSH) $(DEV_SERIAL) $(CHAINBOOT_DEMO_PAYLOAD)

clippy:
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(KERNEL_BIN)

readelf: $(KERNEL_ELF)
	$(call colorecho, "\nLaunching readelf")
	@$(DOCKER_TOOLS) $(READELF_BINARY) --headers $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	$(call colorecho, "\nLaunching objdump")
	@$(DOCKER_TOOLS) $(OBJDUMP_BINARY) --disassemble --demangle \
                --section .text   \
                --section .rodata \
                --section .got    \
                $(KERNEL_ELF) | rustfilt

nm: $(KERNEL_ELF)
	$(call colorecho, "\nLaunching nm")
	@$(DOCKER_TOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt

# For rust-analyzer
check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=human