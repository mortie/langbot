git clone https://github.com/UberPyro/prowl.git
cd prowl
git checkout 0651712a3bd3285d164ac915745ef3905de105f3

opam init
eval `opam env`

opam install -y dune batteries ppx_deriving menhir

dune build
cp _build/default/prowl.exe "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
