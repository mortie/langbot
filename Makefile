GZ ?= pigz
CAT ?= pv

.PHONY: build
build:
	podman build -t langbot .

.PHONY: dist
dist:
	podman build --squash-all -t langbot .
	podman save langbot | $(GZ) | $(CAT) > langbot-image.tgz

.PHONY: load
load: langbot-image.tgz
	$(CAT) $< | $(GZ) -d | podman load

.PHONY: shell
shell:
	podman run --rm -it langbot

.PHONY: run
run:
	@if [ -z "$(L)" ]; then echo "Usage: make run L=whatever" >&2; exit 1; fi
	podman run --rm -it langbot ./scripts/run.sh "$(L)"
