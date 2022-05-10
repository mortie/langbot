cat >input.s
gcc -static -o input input.s
cd wd && exec ../input
