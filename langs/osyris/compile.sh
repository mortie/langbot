git clone https://github.com/mortie/osyris.git
cd osyris
git checkout 0af9b7ad6f4145a88919d61d39f9277ed5db2070

cargo build --release
cp target/release/osyris "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
