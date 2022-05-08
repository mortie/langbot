git clone https://github.com/jfecher/ante.git
cd ante
git checkout 1df3e650de2347b73b46b142e3abb6b87b015356

XDG_CONFIG_HOME="$DEPLOYDIR/.config" cargo build --release
cp target/release/ante "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
