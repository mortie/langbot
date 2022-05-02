# Save code coming from Stdin into a file:
cat > input.ph

# Compile the code:
cd phosphorCompiler
npm start -- -t linuxAmd64 -s ../phosphorStandardLibrary ../input.ph ../output

# Execute the programme:
cd ..
exec ./output
