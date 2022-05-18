git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout f47a0128a59c2e6af9d5cde6e9ecea685a9e1826

cargo build --release
cp target/release/chilic "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
