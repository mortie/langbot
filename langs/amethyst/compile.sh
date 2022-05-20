git clone https://github.com/amethyst-lang/amethyst
cd amethyst
git checkout 885df701942d3d2f0fef04d8d2c24425f43d7984

cargo build --release
cp target/release/amethyst "$DEPLOYDIR"
cp _start.s "$DEPLOYDIR"

nasm -f elf64 _start.s
cp _start.o "$DEPLOYDIR"

./target/release/amethyst self-hosted-compiler/compiler.amy amethystc
cp amethystc "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
