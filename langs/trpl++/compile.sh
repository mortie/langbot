git clone https://git.coredoes.dev/e3team/turbo-rpl
cd turbo-rpl
git checkout dd75f256acce0e8897d1b55369218e987d679693

# Remove call to stty, it won't run in a TTY
sed -i '/system("stty erase/d' main.cpp

g++ -std=c++17 -o trpl main.cpp -pthread

cp trpl "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
