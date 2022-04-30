#!/bin/bash
set -e

concurrency=8
langs='
	ante
	cognate
	cthulhu
	egel
	osyris
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
