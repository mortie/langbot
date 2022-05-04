git clone https://github.com/fabiosvm/hook-lang.git
cd hook-lang
git checkout a1cdd1d248ace61d3fb6fddfaccf87fc482cf742

cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
cp -R ../hook-lang "$DEPLOYDIR"
cp bin/hook "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
