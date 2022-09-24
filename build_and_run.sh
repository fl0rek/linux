#!/bin/bash

set -e

make LLVM=1 -j$(nproc)

usr/gen_init_cpio qemu-initramfs.desc > qemu-initramfs.img

sudo qemu-system-x86_64 \
	-kernel arch/x86/boot/bzImage \
	-initrd qemu-initramfs.img \
	-M pc \
	-m 1G \
	-cpu Cascadelake-Server \
	-smp 1 \
	-nographic \
	-vga none \
	-no-reboot \
	-append 'console=ttyS0'
