# Save code coming from Stdin into a file:
cat >input.ph

if [ -f .unsupported-system ]; then
	echo "Unsupported host system: $(uname -sm)"
	exit 1
fi

# Compile the code:
cd PhosphorCompiler
node bin/main.js -t linuxAmd64 -s ../PhosphorStandardLibrary ../input.ph ../output

if [ ! -f ../output ]; then
	# If no output exists, the compilation did fail and we need to exit with a non-zero exit code:
	exit 1
fi

# Execute the programme:
cd ..
exec ./output
