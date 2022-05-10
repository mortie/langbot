cat >input.lean
./bin/lean --c=input.c input.lean
gcc -I include -o input.o -c input.c

# It's common for lean "programs" to not define a main function,
# just invoke 'eval' in the compile step.
# This generates object files without a main function.
# So we only wanna link and run a proper program if the C code
# generated an object file with a main function.
if objdump -t input.o | grep ' main$'>/dev/null; then
	gcc -o input input.o -L lib/lean -lleanshared -Wl,-rpath="$PWD/lib/lean"
	cd wd && exec ../input
fi
