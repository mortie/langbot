TAG=ghcr.io/mortie/langbot-base:1.0.0

.PHONY: build
build:
	podman build -t $(TAG) .

.PHONY: push
push: build
	podman push $(TAG)
