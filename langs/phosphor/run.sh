# Save code coming from Stdin into a file:
cat > input.ph

# Compile the code:
cd phosphorCompiler
npm start -- -t linuxAmd64 -s ../phosphorStandardLibrary ../input.ph ../output

if [ ! -f ../output ]; then
    # If no output exists, the compilation did fail and we need to exit with a non-zero exit code:
    exit 1
fi

# Execute the programme:
cd ..
exec ./output
