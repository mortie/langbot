#!/bin/bash
set -euo pipefail

lang="$1"

if [ -z "$lang" ]; then
	echo "Usage: $0 <language>"
	exit 1
fi

topdir="$PWD"
deploydir="$topdir/deploy/$lang"

if ! [ -f "$deploydir/.done" ]; then
	echo "Language $lang isn't deployed" >&2
	exit 1
fi

cd "$deploydir"
rm -rf wd
mkdir -p wd
exec bash -euo pipefail run.sh
