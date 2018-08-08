ifeq ($(shell which xargo),)
$(shell cargo install xargo)
endif
ARCH=x86
O:=$(shell realpath build)
SRC=.
SRC_FILES="arch bios Cargo.toml devices iso kernel Makefile mem multiboot"
KERNEL_A=$(O)/$(ARCH)-target/release/libkernel.a
RUST_TARGET_PATH=$(SRC)/arch/$(ARCH)
ifeq ($(shell uname -o),Cygwin)
#O:=$(shell cygpath -am $(O))
#SRC:=$(shell cygpath -am $(SRC))
endif
all: $(O) $(O)/kernel.elf

$(O):
	mkdir $(O)

$(O)/kernel.elf: $(O)/entry.o $(KERNEL_A) $(O)/end.o
	objcopy -O elf32-i386 $(KERNEL_A)
	i686-linux-ld --gc-sections -T $(SRC)/arch/$(ARCH)/link.ld -o $(O)/kernel.elf $(O)/entry.o $(KERNEL_A) $(O)/end.o
	strip $(O)/kernel.elf

$(O)/entry.o: $(SRC)/arch/x86/entry/entry.asm
	nasm -f elf32 $(SRC)/arch/x86/entry/entry.asm -o $(O)/entry.o

$(O)/end.o: $(SRC)/arch/x86/entry/end.asm
	nasm -f elf32 $(SRC)/arch/x86/entry/end.asm -o $(O)/end.o

-include $(O)/$(ARCH)-target/release/libkernel.d
$(KERNEL_A):
	CARGO_TARGET_DIR=$(O) RUST_TARGET_PATH=$(shell realpath $(RUST_TARGET_PATH)) xargo build --target $(ARCH)-target --release

test: $(O)/test.iso 
	#$(QEMU)/qemu-system-i386 -cdrom $(O)/test.iso -bios bios/efi.bin
	#cp $(O)/test.iso /sdcard/
test-pc: $(O)/test.iso
	$(QEMU)/qemu-system-i386 -cdrom $(O)/test.iso
$(O)/test.iso: $(O)/kernel.elf
	cp $(O)/kernel.elf iso/boot
	genisoimage -U -b boot/grub/i386-pc/eltorito.img -no-emul-boot -boot-info-table -eltorito-alt-boot -b boot/grub/efi.img -no-emul-boot -o $(O)/test.iso iso

dist:
	mkdir -p kernel-src
	for i in $(SRC_FILES); do			\
		cp -r $$i kernel-src;	\
	done
	tar -zcvf kernel-src.tgz kernel-src
	rm -r kernel-src

clean:
	rm -r $(O)

.PHONY: all  clean
