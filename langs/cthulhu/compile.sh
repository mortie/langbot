git clone https://github.com/apache-hb/cthulhu.git
cd cthulhu
git checkout 370560eb7c56ea6513bcf8dfbdd11a709f5bbb55

meson build --prefix "$DEPLOYDIR"
ninja -C build
ninja -C build install

touch "$DEPLOYDIR/.done"
