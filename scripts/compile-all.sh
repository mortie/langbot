#!/bin/bash
set -euo pipefail

concurrency=8
langs='
	c
	c++
	ante
	cognate
	cthulhu
	egel
	osyris
	python
'

count=0
for lang in $langs; do
	case "$lang" in '#'*) continue;; esac

	echo "=== START: $lang ==="
	(./scripts/compile.sh "$lang" && echo "=== DONE: $lang ===") &
	count=$((count + 1))
	if [ $count -ge $concurrency ]; then
		wait -n
	fi
done

wait
