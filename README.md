# Langbot

## Creating an image

1. Install podman
2. Build an image: `podman build -t langbot .`

## Running a language

Run `podman run --rm -i langbot ./scripts/run.sh <language>`. It will run the code from stdin.
