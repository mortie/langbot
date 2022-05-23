git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout a44b5e31c8b6469310bbcd172178387b371cdcef

cargo build --release
cp target/release/chilic "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
