cat > input.f95
gfortran -std=f95 -o input input.f95
cd wd && exec ../input
