cat >input.hs
ghc -o input input.hs >/dev/null
cd wd && exec ../input
