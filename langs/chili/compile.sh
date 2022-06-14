git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout 803dc1d4ae1fcd983dd641aae259efbbac4ba74c

cargo build --release
cp target/release/chili "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
