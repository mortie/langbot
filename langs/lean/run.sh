cat >input.lean
./bin/lean --c=output.c input.lean
gcc -I include -o output.o -c output.c

# It's common for lean "programs" to not define a main function,
# just invoke 'eval' in the compile step.
# This generates output files without a main function.
# So we only wanna link and run a proper program if the C code
# generated an object file with a main function.
if objdump -t output.o | grep ' main$'>/dev/null; then
	gcc -o output output.o -L lib/lean -lleanshared -Wl,-rpath="$PWD/lib/lean"
	exec ./output
fi
