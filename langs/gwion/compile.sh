git clone https://github.com/Gwion/Gwion.git -b dev
cd Gwion
git checkout 375fc2720a064801856a82d6339e05bca466b5d1
git submodule update --recursive --init

find . -name soundpipe.h

mkdir -p $DEPLOYDIR/bin $DEPLOYDIR/include $DEPLOYDIR/lib
make -j8 GWPLUG_DIR="$DEPLOYDIR/.gwplug"
make install PREFIX=$DEPLOYDIR

cd plug
cat >list.txt <<EOF
K
Lsys
Machine
Math
Sndfile
Soundpipe
Modules
Std
Tuple
Vecx
EOF
AUTO_INSTALL_DEPS=1 make

mkdir -p "$DEPLOYDIR/.gwplug"
cp */*.so "$DEPLOYDIR/.gwplug"

touch "$DEPLOYDIR/.done"
