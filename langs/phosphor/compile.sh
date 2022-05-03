git clone https://github.com/PhosphorLang/PhosphorStandardLibrary.git
cd PhosphorStandardLibrary
git checkout bfbd217311a6d21c1022a1717c268163ab649997
./build.sh linuxAmd64
cd ..

git clone https://github.com/PhosphorLang/PhosphorCompiler.git
cd PhosphorCompiler
git checkout 74f1a58128201de2627d5954dce70cbb5792a31d
npm install
npm run build
cd ..

cp -r PhosphorStandardLibrary/bin "$DEPLOYDIR/PhosphorStandardLibrary"
cp -r PhosphorCompiler "$DEPLOYDIR"

# Sanity confirmation:
touch "$DEPLOYDIR/.done"
