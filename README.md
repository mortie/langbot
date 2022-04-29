# Langbot

## Creating a pod with macOS

1. Download podman: `brew install podman`
2. Set up a virtual machine: `podman machine init --cpus 8 --memory 8096`
3. Start it up: `podman machine start`
4. Build the image: `podman build -t langbot .`
5. Start the container: `podman run --rm -it langbot bash`
6. Compile language implementations: `./scripts/compile-all.sh`
