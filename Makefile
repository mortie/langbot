GZ ?= pigz
CAT ?= pv

.PHONY: build
build:
	mkdir -p staging
	podman build --volume "$(abspath staging)":/app/staging -t langbot .

.PHONY: dist
dist: build
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
	podman run --rm -i langbot ./scripts/run.sh "$(L)"

.PHONY: check
check:
	./scripts/run-tests.sh
