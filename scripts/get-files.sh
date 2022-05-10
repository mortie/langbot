#!/bin/bash
set -euo pipefail

lang="$1"

if [ -z "$lang" ]; then
	echo "Usage: $0 <language>"
	exit 1
fi

topdir="$PWD"
deploydir="$topdir/deploy/$lang"

if [ -d "$deploydir/wd" ]; then
	(cd "$deploydir/wd" && tar c .)
fi
