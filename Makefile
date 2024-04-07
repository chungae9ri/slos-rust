RUSTC = rustc
TARGET = aarch64-unknown-none

#define make-obj
#$1/%.o: %.rs
#	$(RUSTC) --emit=obj --target=armv7a-none-eabi -gg -o $$@ $$<
#
#$1/%.o: %.S
#$(CC) $(INC) -o $$@ -c $$< -g -mcpu=cortex-a9 -mfpu=vfpv3 -mfloat-abi=hard -Wall -fno-omit-frame-pointer -ffreestanding
#endef
#
#$(foreach bdir, $(KERNOUTDIR),$(eval $(call make-obj,$(bdir))))

LD_SCRIPT_PATH    = $(shell pwd)/src/linker

RUSTFLAGS = -C target-cpu=cortex-a53
##--------------------------------------------------------------------------------------------------
## Targets and Prerequisites
##--------------------------------------------------------------------------------------------------
KERNEL_MANIFEST      = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel64.ld
LAST_BUILD_CONFIG    = target/$(TARGET).build_config

KERNEL_ELF      = target/$(TARGET)/debug/kernel64

RUSTFLAGS = $(RUSTFLAGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

# This parses cargo's dep-info file.
# https://doc.rust-lang.org/cargo/guide/build-cache.html#dep-info-files
KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST) $(LAST_BUILD_CONFIG)

RUSTC_BUILD_ARGS = --features man_registers --target $(TARGET)

RUSTC_BUILD = cargo rustc $(RUSTC_BUILD_ARGS)

##--------------------------------------------------------------------------------------------------
## Targets
##--------------------------------------------------------------------------------------------------
.PHONY: all clean

all: $(KERNEL_ELF)

##------------------------------------------------------------------------------
## Compile the kernel ELF
##------------------------------------------------------------------------------
$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	$(RUSTC_BUILD)
	mv $@ $@.elf

##------------------------------------------------------------------------------
## Save the configuration as a file, so make understands if it changed.
##------------------------------------------------------------------------------
$(LAST_BUILD_CONFIG):
	@rm -f target/*.build_config
	@mkdir -p target
	@touch $(LAST_BUILD_CONFIG)

clean:
	rm -rf target
