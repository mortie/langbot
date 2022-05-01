git clone https://github.com/mortie/gilia.git
cd gilia
git checkout e302a546fd06b53ab7e43464248c5f90a8931951

make -j 8
cp build/gilia "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
