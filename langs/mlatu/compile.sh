git clone https://github.com/mlatu-lang/mlatu-runner.git
cd mlatu-runner
git checkout 3974c0e501f80e2633bd7b47ee931e571f38f2a4

cargo build --release
cp target/release/mlatu-runner "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
