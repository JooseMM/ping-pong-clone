nasm -f bin boot.asm -o boot.bin
qemu-system-i386 -kernel yourfile.bin
