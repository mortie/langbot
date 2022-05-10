cat > input.ct

# ctc is kinda noisy. Errors go to stderr,
# the noise goes to stdout, so redirecting stdout is ok
./bin/ctc -gen c99 -o input input.ct >/dev/null
gcc -w -o input input.c
cd wd && exec ../input
