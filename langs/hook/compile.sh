git clone https://github.com/fabiosvm/hook-lang.git
cd hook-lang
git checkout 15cf5da8d0d7bbc51b792e97b4c0b11a2df20a06

cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
cp -R ../hook-lang "$DEPLOYDIR"
cp bin/hook "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
