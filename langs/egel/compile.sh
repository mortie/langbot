git clone https://github.com/egel-lang/egel.git
cd egel
git checkout 8dd5a50382f9e898128581a04e2f49abbe3fa131

# Egil is missing an include
sed -i "2 i #include <atomic>" src/runtime.hpp

mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX="$DEPLOYDIR"
make -j8
make install

touch "$DEPLOYDIR/.done"
