#!/bin/bash
set -euo pipefail

concurrency=8
langs='
	ante
	asm
	c
	c++
	carbon
	cognate
	cthulhu
	egel
	fortran
	gilia
	haskell
	javascript
	nasm
	osyris
	phosphor
	python
	racket
	ruby
	rust
	shell
'

count=0
for lang in $langs; do
	case "$lang" in '#'*) continue;; esac

	./scripts/compile.sh "$lang" &
	count=$((count + 1))
	if [ $count -ge $concurrency ]; then
		wait -n
	fi
done

wait

failed=0
for lang in $langs; do
	if ! [ -f "deploy/$lang/.done" ]; then
		echo "Language $lang didn't get deployed!" >&2
		failed=1
	fi
done
if [ "$failed" = 1 ]; then
	exit 1
fi
