git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout b4841d55333a88be36415207f69a8f30e5251f0c

make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
