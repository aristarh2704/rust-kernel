all: vmlinux
vmlinux: entry.o
	cargo build --release
	ld -m i386pe -T link.ld -o vmlinux entry.o target/release/deps/libkernel*.rlib target/release/deps/libconsole*.rlib

vmlinux.elf: vmlinux
	objcopy -O elf32-i386 vmlinux vmlinux.elf

entry.o: entry.asm
	nasm -f win32 entry.asm -o entry.o

test: vmlinux.elf
ifeq ($(QEMU),)
	$(error $$QEMU must be set to qemu dir path)
else
	$(QEMU)/qemu-system-i386 -kernel vmlinux.elf
endif

clean:
	rm -f vmlinux *.o
	cargo clean

.PHONY: all test clean vmlinux

