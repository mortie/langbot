git clone https://github.com/egel-lang/egel.git
cd egel
git checkout 996666143940a9bab97d0324610e866c08b0bbcf

# Egil is missing an include
sed -i "2 i #include <atomic>" src/runtime.hpp

mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX="$DEPLOYDIR"
make -j8
make install

touch "$DEPLOYDIR/.done"
