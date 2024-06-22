RUSTC = rustc
TARGET = aarch64-unknown-none

LD_SCRIPT_PATH    = $(shell pwd)/src/linker

KERNEL_MANIFEST      = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel64.lds

KERNEL_ELF      = target/$(TARGET)/debug/kernel64-rust

RUSTFLAGS_APPEND = -C target-cpu=cortex-a53 \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT) \
	-C link-arg=-Map=$(KERNEL_ELF).map

KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST)

RUSTC_BUILD_ARGS = --features man_registers --target $(TARGET)

RUSTC_BUILD = cargo rustc $(RUSTC_BUILD_ARGS)

.PHONY: all clean

all: $(KERNEL_ELF)

$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	@RUSTFLAGS="$(RUSTFLAGS_APPEND)" $(RUSTC_BUILD)
	mv $@ $@.elf

clean:
	rm -rf target
