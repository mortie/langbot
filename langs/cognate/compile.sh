git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout e88c5bfbb38929718e708ac39c81bae8eaa130a7

make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
