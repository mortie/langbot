git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout 25a49c4150c095cb395efa97044406ad4ba7f418

make PREFIX="$DESTDIR" DESTDIR= install
cp "$WORKDIR/run.sh" "$DESTDIR/run"
touch "$DESTDIR/.done"
