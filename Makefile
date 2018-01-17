RUST_TARGET_PATH:=`cygpath -m -a ./`

all: vmlinux
vmlinux: entry.o
	RUST_TARGET_PATH=$(RUST_TARGET_PATH) xargo build --target kernel --release
	ld -m i386pe --gc-sections -T link.ld -o vmlinux entry.o target/kernel/release/libkernel.a

vmlinux.elf: vmlinux
	objcopy -O elf32-i386 vmlinux vmlinux.elf

entry.o: entry.asm
	nasm -f win32 entry.asm -o entry.o

check:
ifeq (`which xargo`,)
	$(error You must install xargo, run "cargo install xargo")
endif

test: check vmlinux.elf
ifeq ($(QEMU),)
	$(error $$QEMU must be set to qemu dir path)
else
	$(QEMU)/qemu-system-i386 -kernel vmlinux.elf
endif

clean:
	rm -f vmlinux *.o
	cargo clean

.PHONY: all test clean vmlinux check

