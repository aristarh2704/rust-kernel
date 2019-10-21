cmake_minimum_required(VERSION 3.15)
project(RustKernel NONE)
find_program(
	NASM
	nasm
	DOC "nasm assembler"
	)
add_custom_command(OUTPUT build
	COMMAND mkdir build
	)
add_custom_command(OUTPUT build/entry.o
	DEPENDS ${CMAKE_SOURCE_DIR}/arch/x86/entry/entry.asm
	COMMAND nasm -f elf32 ${CMAKE_SOURCE_DIR}/arch/x86/entry/entry.asm -o build/entry.o
	)
add_custom_command(OUTPUT build/x86-target/release/libkernel.a
	COMMAND CARGO_TARGET_DIR=build RUST_TARGET_PATH=${CMAKE_SOURCE_DIR}/arch/x86 xargo build --target x86-target --release --manifest-path=${CMAKE_SOURCE_DIR}/Cargo.toml
	)
add_custom_command(OUTPUT build/kernel.elf
	COMMAND ld.lld  --gc-sections -T ${CMAKE_SOURCE_DIR}/arch/x86/link.ld -o build/kernel.elf build/entry.o build/x86-target/release/libkernel.a
	COMMAND llvm-strip --strip-unneeded build/kernel.elf
	DEPENDS build/entry.o build/x86-target/release/libkernel.a)
add_custom_command(OUTPUT build/test.iso
	WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
	DEPENDS build
	DEPENDS build/kernel.elf
	COMMAND	xorriso -as mkisofs -U -b boot/grub/i386-pc/eltorito.img -no-emul-boot -boot-info-table -eltorito-alt-boot -b boot/grub/efi.img -no-emul-boot -o ${PROJECT_BINARY_DIR}/build/test.iso iso
	)
add_custom_target(TestIso ALL
	DEPENDS build/test.iso
	)
