git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout 99f8fafc71346634bae57eaef43d3b6ed32a7beb

make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
