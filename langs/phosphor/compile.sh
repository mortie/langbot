mkdir download
cd download

# Download the standard library:
curl -L https://github.com/PhosphorLang/PhosphorStandardLibrary/releases/download/version%2F0.1/phosphorStandardLibrary.tgz -o phosphorStandardLibrary.tgz
tar zxf phosphorStandardLibrary.tgz

# Download the compiler:
curl -L https://github.com/PhosphorLang/PhosphorCompiler/releases/download/version%2F0.2.1/phosphor-compiler-0.2.1.tgz -o phosphor-compiler.tgz
tar zxf phosphor-compiler.tgz
mv package phosphorCompiler

# Deploy:
cp -r phosphorStandardLibrary "$DEPLOYDIR"
cp -r phosphorCompiler "$DEPLOYDIR"

# Sanity confirmation:
touch "$DEPLOYDIR/.done"
