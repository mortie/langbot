#!/bin/bash
set -euo pipefail

lang="$1"

if [ -z "$lang" ]; then
	echo "Usage: $0 <language>"
	exit 1
fi

topdir="$PWD"
langdir="$topdir/langs/$lang"

if ! [ -d "$langdir" ]; then
	echo "Unknown language: $lang" >&2
	exit 1
fi

if ! [ -f "$langdir/run.sh" ]; then
	echo "Missing file: $langdir/run.sh" >&2
	exit 1
fi

# Compute the shasum of the input files, for change tracking
shasum="$(find "$langdir" -type f -exec shasum {} \; | shasum)"

# Do we need to recompile?
if \
	! [ -f "$topdir/staging/$lang/.shasum" ] || \
	[ "$(cat "$topdir/staging/$lang/.shasum")" != "$shasum" ]
then
	echo "=== START: $lang ===" >&2

	# Create workdir, populate it with the content of the language spec dir
	rm -rf "$topdir/work/$lang"
	mkdir -p "$topdir/work"
	cp -r "$langdir" "$topdir/work"

	# Create staging dir, populate it with run.sh if it exists
	rm -rf "$topdir/staging/$lang"
	mkdir -p "$topdir/staging/$lang"
	if [ -e "$topdir/work/$lang/run.sh" ]; then
		cp "$topdir/work/$lang/run.sh" "$topdir/staging/$lang"
	fi

	# If there's a compile.sh script, run it, with DEPLOYDIR set to the staging dir
	if [ -f "$topdir/work/$lang/compile.sh" ]; then
		(cd "$topdir/work/$lang" && \
			DEPLOYDIR="$topdir/staging/$lang" bash -x -euo pipefail ./compile.sh)
	else
		touch "$topdir/staging/$lang/.done"
	fi

	# 'compile.sh' scripts should produce a .done file as a sanity check
	if ! [ -f "$topdir/staging/$lang/.done" ]; then
		echo "$langdir/compile.sh failed to produce a '$$DEPLOYDIR/.done' file!" >&2
		exit 1
	fi

	# Store the shasum of the input files, to avoid recompiling later
	echo "$shasum" >"$topdir/staging/$lang/.shasum"

	echo "=== DONE: $lang ===" >&2
else
	echo "Nothing to do for $lang." >&2
fi

# Create deploy dir, populate it with the content of the staging dir
rm -rf "$topdir/deploy/$lang"
mkdir -p "$topdir/deploy"
cp -r "$topdir/staging/$lang" "$topdir/deploy"
