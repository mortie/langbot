git clone https://github.com/shadowninja55/carbon.git
cd carbon
git checkout 51b60368c5f6b7d2cf9850d035eb3840a77fd9c8

cabal build
cp dist-newstyle/build/*/*/*/x/carbon/build/carbon/carbon "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
