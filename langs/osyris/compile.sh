git clone https://github.com/mortie/osyris.git
cd osyris
git checkout 0c6c6dbbb96a6b038b91d47541c8924ef3a2b74b

cargo build --release
cp target/release/osyris "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
