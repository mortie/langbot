git clone https://github.com/mortie/osyris.git
cd osyris
git checkout 2db1f2c1746dbda2451d8cc888921a0f518aaf78

cargo build --release
cp target/release/osyris "$DESTDIR"

touch "$DESTDIR/.done"
