yasm -f elf64 -o output.o output.asm
ld output.o -o output

./output
