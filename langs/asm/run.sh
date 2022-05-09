cat >input.s
gcc -static -o output input.s
exec ./output
