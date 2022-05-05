git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout 3e4066eb8b1da15b18270a0f748b37964bfba0cf

make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
