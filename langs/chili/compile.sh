git clone https://github.com/r0nsha/chili-lang.git --recurse-submodules chili
cd chili
git checkout 5306da368395279ec9620f178f920fd250d5bc37

cargo build --release
cp target/release/chili "$DEPLOYDIR"
cp -R lib "$DEPLOYDIR/lib"

touch "$DEPLOYDIR/.done"
