#!/bin/sh
mv "$1" "$1.cog"
./bin/cognac "$1.cog" -run
