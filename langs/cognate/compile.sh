git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout bce8602b55d4dbd83cd91d7d115307f9e8470bf4

make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
