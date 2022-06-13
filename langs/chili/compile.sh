git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout 962ac5d625d49f9089b1220271253c0817c9698d

cargo build --release
cp target/release/chili "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
