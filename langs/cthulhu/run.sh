echo '@mangle("printf")
def print(fmt: str, args: ...): int;
' > input.ct
cat >> input.ct

./bin/ctc -o input input.ct
gcc -o input.bin input.c
exec ./input.bin
