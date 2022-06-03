git clone https://github.com/UberPyro/prowl.git
cd prowl
git checkout 6c108913ad3e66e7dd9741ad500a27a3ba75906b

opam init
eval `opam env`

opam install -y dune batteries ppx_deriving menhir alcotest

dune build
cp _build/default/prowl.exe "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
