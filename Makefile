RUSTC = rustc
TARGET = aarch64-unknown-none

LD_SCRIPT_PATH    = $(shell pwd)/src/linker

KERNEL_MANIFEST      = Cargo.toml
ZYNQMP_LDS = kernel_zynqmp.lds
RPI4_LDS = kernel_rpi4.lds

ZYNQMP_ELF      = target/$(TARGET)/debug/kernel-zynqmp-rust
RPI4_ELF      = target/$(TARGET)/debug/kernel-rpi4-rust

ZYNQMP_RUSTFLAGS_APPEND = -C target-cpu=cortex-a53 \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(ZYNQMP_LDS) \
	-C link-arg=-Map=$(ZYNQMP_ELF).map

RPI4_RUSTFLAGS_APPEND = -C target-cpu=cortex-a53 \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(RPI4_LDS) \
	-C link-arg=-Map=$(RPI4_ELF).map

ZYNQMP_ELF_DEPS = $(filter-out %: ,$(file < $(ZYNQMP_ELF).d)) $(KERNEL_MANIFEST)
RPI4_ELF_DEPS = $(filter-out %: ,$(file < $(RPI4_ELF).d)) $(KERNEL_MANIFEST)

RUSTC_BUILD_ARGS = --target $(TARGET)

RUSTC_BUILD = cargo rustc $(RUSTC_BUILD_ARGS)

.PHONY: all clean

all: $(ZYNQMP_ELF) $(RPI4_ELF)

$(ZYNQMP_ELF): $(ZYNQMP_ELF_DEPS)
	RUSTFLAGS="$(ZYNQMP_RUSTFLAGS_APPEND)" $(RUSTC_BUILD) --bin kernel-zynqmp-rust
	mv -f $@ $@.elf

$(RPI4_ELF): $(RPI4_ELF_DEPS)
	RUSTFLAGS="$(RPI4_RUSTFLAGS_APPEND)" $(RUSTC_BUILD) --bin kernel-rpi4-rust
	mv -f $@ $@.elf

clean:
	rm -rf target
