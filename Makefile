run:
	cargo build
	mkdir -p iso/boot
	cp target/x86_64-unknown-none/release/obsidian-kernel iso/boot/obsidian-kernel.elf
	grub-mkrescue -o obsidian.iso iso
	qemu-system-x86_64 -cdrom obsidian.iso