cat >input.amy
./amethyst input.amy &> /dev/null && cd wd && exec ../a.out
./amethystc input.amy
exit 1

