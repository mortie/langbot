git clone https://github.com/leanprover/lean4.git
cd lean4
git checkout 96de208a6b1a575b3f07d978965573d5b6fad454
git submodule update --init

mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX="$DEPLOYDIR" -DCMAKE_BUILD_TYPE=Release
cmake --build . -j 8
cmake --install .

touch "$DEPLOYDIR/.done"
