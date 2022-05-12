git clone https://github.com/Gwion/Gwion.git -b dev
cd Gwion
git checkout dfbd2753b29346712327075fac2579804ac52a6d
git submodule update --recursive --init

find . -name soundpipe.h

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
Sporth
Stk
FMSynth
BMI
EOF
AUTO_INSTALL_DEPS=1 make

mkdir -p "$DEPLOYDIR/.gwplug"
cp */*.so "$DEPLOYDIR/.gwplug"

touch "$DEPLOYDIR/.done"
