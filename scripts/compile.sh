#!/bin/bash
set -e

lang="$1"

if [ -z "$lang" ]; then
	echo "Usage: $0 <language>"
	exit 1
fi

topdir="$PWD"
langdir="$topdir/langs/$lang"
workdir="$topdir/work/$lang"
deploydir="$topdir/deploy/$lang"

if ! [ -d "$langdir" ]; then
	echo "Unknown language: $lang" >&2
	exit 1
fi

if ! [ -f "$langdir/compile.sh" ]; then
	echo "Missing file: $langdir/compile.sh" >&2
	exit 1
fi

if ! [ -f "$langdir/run.sh" ]; then
	echo "Missing file: $langdir/run.sh" >&2
	exit 1
fi

rm -rf "$deploydir"
rm -rf "$workdir"
mkdir -p "$deploydir"
mkdir -p "$topdir/work"
cp -r "$langdir" "$workdir"

if [ -e "$workdir/run.sh" ]; then
	cp "$workdir/run.sh" "$deploydir/run.sh"
fi

cd "$workdir"
if ! WORKDIR="$workdir" DESTDIR="$deploydir" bash -ex "$langdir/compile.sh"; then
	echo "Failed to compile $lang." >&2
	exit 1
fi

if ! [ -f "$deploydir/.done" ]; then
	echo "Compile script failed to produce $deploydir/.done file." >&2
	exit 1
fi

if ! [ -f "$deploydir/run.sh" ]; then
	echo "Compile script failed to produce $deploydir/run.sh file." >&2
	exit 1
fi

echo "Compiled $lang." >&2
