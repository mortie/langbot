git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout 24d08d24d420046a7ce1ed9285a996cc1c9961e9

cargo build --release
cp target/release/chili "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
