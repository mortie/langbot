#!/bin/bash
set -e

langs="
	ante
	cognate
	cthulhu
	egel
	osyris
"

concurrency=4

count=0
for lang in $langs; do
	echo "=== START: $lang ==="
	(./scripts/compile.sh "$lang" && echo "=== DONE: $lang ===") &
	count=$((count + 1))
	if [ $count -ge $concurrency ]; then
		wait -n
	fi
done

wait
