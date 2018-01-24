ARCH=x86
O=build
SRC=$(shell realpath ./)
KERNEL_A=$(O)/target/$(ARCH)-target/release/libkernel.a
RUST_TARGET_PATH=$(SRC)/arch/$(ARCH)

ifeq ($(shell uname -o),Cygwin)
O:=$(shell cygpath -am $(O))
SRC:=$(shell cygpath -am $(SRC))
endif
all: $(O)/vmlinux.elf
	echo $(RUST_TARGET_PATH)
	echo $(O)

$(O)/vmlinux.elf: $(O)/entry.o $(KERNEL_A)
	ld -m i386pe --gc-sections -T $(SRC)/arch/$(ARCH)/link.ld -o $(O)/vmlinux $(O)/entry.o $(KERNEL_A)
	objcopy -O elf32-i386 vmlinux vmlinux.elf

$(O)/entry.o: $(SRC)/arch/x86/entry/entry.asm
	nasm -f win32 $(SRC)/arch/x86/entry/entry.asm -o $(O)/entry.o

-include $(O)/target/$(ARCH)-target/release/libkernel.d
$(KERNEL_A):
	cd $(O)
	RUST_TARGET_PATH=$(RUST_TARGET_PATH) xargo build --target $(ARCH)-target --release
	cd $(SRC)		

.PHONY: all