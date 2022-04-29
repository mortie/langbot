#!/bin/bash
set -e

langs="
	ante
	cognate
	cthulhu
	egel
	osyris
"

for lang in $langs; do
	./scripts/compile.sh "$lang"
done
