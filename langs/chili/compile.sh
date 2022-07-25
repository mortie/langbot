git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout 505151a99abe6ba996bdd5a0282ceebd592c8a6c

cargo build --release
cp target/release/chili "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
