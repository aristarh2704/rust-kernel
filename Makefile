ARCH=x86
O=build
SRC=$(shell realpath ./)
SRC_FILES="arch Cargo.* kernel mem multiboot Makefile"
KERNEL_A=$(O)/$(ARCH)-target/release/libkernel.a
RUST_TARGET_PATH=$(SRC)/arch/$(ARCH)

ifeq ($(shell uname -o),Cygwin)
O:=$(shell cygpath -am $(O))
SRC:=$(shell cygpath -am $(SRC))
endif
all: $(O) $(O)/vmlinux.elf

$(O):
	mkdir $(O)

$(O)/vmlinux.elf: $(O)/entry.o $(KERNEL_A) $(O)/end.o
	ld -m i386pe --gc-sections -T $(SRC)/arch/$(ARCH)/link.ld -o $(O)/vmlinux $(O)/entry.o $(KERNEL_A) $(O)/end.o
	objcopy -O elf32-i386 $(O)/vmlinux $(O)/vmlinux.elf
	strip $(O)/vmlinux.elf

$(O)/entry.o: $(SRC)/arch/x86/entry/entry.asm
	nasm -f win32 $(SRC)/arch/x86/entry/entry.asm -o $(O)/entry.o

$(O)/end.o: $(SRC)/arch/x86/entry/end.asm
	nasm -f win32 $(SRC)/arch/x86/entry/end.asm -o $(O)/end.o

-include $(O)/$(ARCH)-target/release/libkernel.d
$(KERNEL_A):
	CARGO_TARGET_DIR=$(O) RUST_TARGET_PATH=$(RUST_TARGET_PATH) xargo build --target $(ARCH)-target --release

test: all $(O)/vmlinux.elf
	$(QEMU)/qemu-system-i386 -kernel $(O)/vmlinux.elf

dist:
	mkdir -p kernel-src
	for i in $(SRC_FILES); do			\
		cp -r $$i kernel-src;	\
	done
	tar -zcvf kernel-src.tgz kernel-src
	rm -r kernel-src

.PHONY: all $(KERNEL_A)
