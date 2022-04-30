echo '@mangle("printf")
def print(fmt: str, args: ...): int;
' > input.ct
cat >> input.ct

# ctc is kinda noisy. Errors go to stderr,
# the noise goes to stdout, so redirecting stdout is ok
./bin/ctc -gen c99 -o input input.ct >/dev/null
gcc -w -o input.bin input.c
exec ./input.bin
