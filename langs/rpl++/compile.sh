git clone https://git.coredoes.dev/e3team/rpl.git
cd rpl
git checkout 32d94c271f817bf1471ebfc0ceacbda61b679cb1
cd ..

cp -r rpl "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
