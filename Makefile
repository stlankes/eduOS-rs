arch ?= aarch64
target ?= $(arch)-eduos
release ?=

arch ?= x86_64
target ?= $(arch)-unknown-none-gnu
release ?=

ifeq ($(release), 1)
opt := --release
rdir := release
endif

rust_os := target/$(target)/$(rdir)/libeduos_rs.a
kernel := build/kernel-$(arch)

crossprefix :=
uname_s := $(shell uname -s)


build_wasm :=
ifeq ($(arch), wasm32)
build_wasm := eduos.wasm
endif

RN :=
ifdef COMSPEC
RM := del
else
RM := rm -rf
endif

.PHONY: all fmt clean run debug cargo docs

bootimage.bin:
	bootimage build --target $(target).json

fmt:
	rustfmt --write-mode overwrite src/lib.rs

qemu: # bootimage.bin
	@qemu-system-$(arch) -machine virt -display none -smp 1 -serial stdio -drive format=raw,file=bootimage.bin || true


run: $(kernel).elf
	@echo QEMU $(kernel).elf
	@qemu-system-x86_64 -display none -smp 1 -net nic,model=rtl8139 -device isa-debug-exit,iobase=0xf4,iosize=0x04 -monitor telnet:127.0.0.1:18767,server,nowait -kernel $(kernel).elf -serial stdio 2>/dev/null || true

debug: $(kernel).elf
	@echo QEMU -d int $(kernel).elf
	@qemu-system-x86_64 -display none -smp 1 -net nic,model=rtl8139 -device isa-debug-exit,iobase=0xf4,iosize=0x04 -monitor telnet:127.0.0.1:18767,server,nowait -kernel $(kernel).elf -d int -no-reboot -serial stdio

docs:
	@echo DOC
	@cargo doc

docs:
	@echo DOC
	@cargo doc

cargo:
	@echo CARGO
	@cargo xbuild $(opt) --target $(target).json


build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm $(assembly_header_files)
	@echo NASM $<
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 -Isrc/arch/$(arch)/ $< -o $@


#==========================================================================
# Building the Rust runtime for our bare-metal target

# Where to put our compiled runtime libraries for this platform.
installed_target_libs := \
	$(shell rustup which rustc | \
		sed s,bin/rustc,lib/rustlib/$(target)/lib,)

runtime_rlibs := \
	$(installed_target_libs)/libcore.rlib \
	$(installed_target_libs)/libstd_unicode.rlib \
	$(installed_target_libs)/liballoc.rlib

RUSTC := \
	rustc --verbose --target $(target) \
		-Z no-landing-pads \
		--out-dir $(installed_target_libs)

.PHONY: runtimea

runtime: $(runtime_rlibs)

$(installed_target_libs):
	@mkdir -p $(installed_target_libs)

$(installed_target_libs)/%.rlib: rust/src/%/lib.rs $(installed_target_libs)
	@echo RUSTC $<
	@$(RUSTC) --crate-type rlib --crate-name $(shell basename $@ | sed s,lib,, | sed s,.rlib,,) $<
	@echo Check $@
