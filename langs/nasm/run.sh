cat >input.s
nasm -f elf64 -o input.o input.s
ld -o output input.o
exec ./output
