RUST_TARGET_PATH:=$(shell cygpath -m -a ./)
SRC_FILES="Cargo.* console entry.asm kernel.json kernel.rs link.ld Makefile"
KERNEL_A:=$(shell cygpath -w -a target/kernel/release/libkernel.a)

all: check vmlinux.elf

vmlinux.elf: entry.o $(KERNEL_A)
	ld -m i386pe --gc-sections -T link.ld -o vmlinux entry.o target/kernel/release/libkernel.a
	objcopy -O elf32-i386 vmlinux vmlinux.elf

-include target\kernel\release\libkernel.d
$(KERNEL_A):
	RUST_TARGET_PATH=$(RUST_TARGET_PATH) xargo build --target kernel --release

entry.o: entry.asm
	nasm -f win32 entry.asm -o entry.o

check:
ifeq (`which xargo`,)
	$(error You must install xargo, run "cargo install xargo")
endif

dist:
	mkdir -p kernel-src
	for i in $(SRC_FILES); do \
		cp -r $$i kernel-src;\
	done
	tar -zcf kernel-src.tgz kernel-src
	rm -r kernel-src

test: vmlinux.elf
ifeq ($(QEMU),)
	$(error $$QEMU must be set to qemu dir path)
else
	$(QEMU)/qemu-system-i386 -kernel vmlinux.elf
endif

clean:
	rm -f vmlinux *.o
	cargo clean

.PHONY: all test clean check dist

