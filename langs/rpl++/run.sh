# RPL is odd and will chdir to the directory its file is in,
# so input.rpl has to be in the wd.
# That also means we have to clean up input.rpl after exit
# to avoid uploading it.
cd wd
cat >input.rpl
if node ../rpl input.rpl; then
	ret=0
else
	ret=$?
fi
rm -f input.rpl
exit $ret
