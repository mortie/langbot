git clone https://github.com/StavromulaBeta/cognate.git
cd cognate
git checkout fe37c6c5f35c51f2722618450ec3d602a1f2760a

make PREFIX="$DEPLOYDIR" install

touch "$DEPLOYDIR/.done"
