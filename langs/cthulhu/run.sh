#!/bin/sh
echo '@mangle("printf")
def print(fmt: str, args: ...): int;
' > "$1.ct"
cat "$1" >> "$1.ct"
./bin/ctc -o "$1" "$1.ct"
gcc -o "$1.bin" "$1.c"
"$(readlink -f "$1.bin")"
