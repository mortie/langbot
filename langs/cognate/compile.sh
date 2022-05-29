git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout 2158e41c0902c72b13a37337998755fc2012e756
make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
