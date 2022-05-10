#!/bin/bash
set -euo pipefail

failed=0

run_lang() {
	podman run --rm -i langbot ./scripts/run.sh "$1"
}

machine="$(uname -m)"
os="$(uname -s)"

for lang in $(ls langs); do
	if [ -f "langs/$lang/hello-world.txt" ]; then
		f="langs/$lang/hello-world.txt"
	elif [ -f "langs/$lang/hello-world.$machine.txt" ]; then
		f="langs/$lang/hello-world.$machine.txt"
	elif [ -f "langs/$lang/hello-world.$os-$machine.txt" ]; then
		f="langs/$lang/hello-world.$os-$machine.txt"
	else
		echo "   Warn: $lang: No hello-world.txt file for $os-$machine"
		continue
	fi

	if [ -f "$f" ]; then
		if output="$(cat "$f" | run_lang "$lang")"; then
			: # OK
		else
			echo "   Fail: $lang: Non-zero exit code $?"
			echo "stdout:"
			echo "$output" | sed 's/^/> /'
			failed=1
			continue
		fi

		if [ "$output" != "Hello World" ]; then
			echo "   Fail: $lang: Wrong output, expected 'Hello World'"
			echo "stdout:"
			echo "$output" | sed 's/^/> /'
			failed=1
			continue
		fi

		echo "Success: $lang"
	fi
done

if [ "$failed" = 0 ]; then
	echo "Ok!"
else
	echo "Err!"
	exit 1
fi
