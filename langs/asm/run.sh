cat >input.s
as -o input.o input.s
ld -o output input.o
exec ./output
