curl -o lua.tar.gz http://www.lua.org/ftp/lua-5.4.4.tar.gz
mkdir lua-src
tar xf lua.tar.gz --strip-components=1 -C lua-src
cd lua-src

make -j$(nproc)
cp src/lua "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
