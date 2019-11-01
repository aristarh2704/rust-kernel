KERNEL_A:=$(shell realpath build/target/release/libkernel.a)
all: build/test.iso
build/kernel.elf: $(KERNEL_A) build/entry.o
	ld.lld --gc-sections -T arch/x86/link.ld -o build/kernel.elf build/entry.o build/target/release/libkernel.a
-include build/target/release/libkernel.d
$(KERNEL_A):
	CARGO_TARGET_DIR=build cargo xbuild --target=arch/x86/target.json --release
build/entry.o: arch/x86/entry.asm
	yasm -p nasm -f elf32 arch/x86/entry.asm -o build/entry.o
build/test.iso: build/kernel.elf
	cp -r iso build/
	cp build/kernel.elf build/iso/boot
	genisoimage -U -b boot/grub/i386-pc/eltorito.img -no-emul-boot -boot-info-table -eltorito-alt-boot -b boot/grub/efi.img -no-emul-boot -o build/test.iso build/iso
test: build/test.iso
	qemu-system-i386 -cdrom build/test.iso -display none -serial file:a

