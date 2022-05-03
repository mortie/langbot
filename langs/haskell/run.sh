cat >input.hs
ghc -o output input.hs >/dev/null
exec ./output
