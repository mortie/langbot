git clone https://github.com/mlatu-lang/mlatu-runner.git
cd mlatu-runner
git checkout eedc8cbb98b840aca3492ffacce87032a570f718

cargo build --release
cp target/release/mlatu-runner "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
