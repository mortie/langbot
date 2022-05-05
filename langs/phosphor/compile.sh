git clone https://github.com/PhosphorLang/PhosphorStandardLibrary.git
cd PhosphorStandardLibrary
git checkout de8733a71b13946380646cffe44a68483b91575b
./build.sh linuxAmd64
cd ..

git clone https://github.com/PhosphorLang/PhosphorCompiler.git
cd PhosphorCompiler
git checkout 1015e9c3a15487ceb238fc1f8e250a1e79a56cf2
npm install
npm run build
cd ..

cp -r PhosphorStandardLibrary/bin "$DEPLOYDIR/PhosphorStandardLibrary"
cp -r PhosphorCompiler "$DEPLOYDIR"

# Sanity confirmation:
touch "$DEPLOYDIR/.done"
