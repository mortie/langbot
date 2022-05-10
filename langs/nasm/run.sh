cat >input.s
nasm -f elf64 -o input.o input.s
gcc -static -o input input.o
cd wd && exec ../input
