cat >input.ante
export XDG_CONFIG_HOME="$PWD/.config"
cd wd && exec ../ante --run input.ante
