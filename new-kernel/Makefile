KERNEL_A:=$(shell realpath build/target/release/libkernel.a)
all: build/test.iso
build/kernel.elf: $(KERNEL_A) build/entry.o
	ld.lld --gc-sections -T arch/x86/link.ld -o build/kernel.elf build/entry.o build/target/release/libkernel.a
-include build/target/release/libkernel.d
$(KERNEL_A):
	CARGO_TARGET_DIR=build cargo xbuild --target=arch/x86/target.json --release
build/entry.o: arch/x86/entry.asm
	yasm -p nasm -f elf32 arch/x86/entry.asm -o build/entry.o
build/iso: build/kernel.elf
	cp -r iso build/
	cp build/kernel.elf build/iso/boot
test: build/iso
	qemu-system-i386  -boot n -device e1000,netdev=n1 -netdev user,id=n1,tftp=build/iso/,bootfile=/boot/grub/i386-pc/core.0 -display none -serial file:a

