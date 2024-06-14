PROJECT_ROOT := $(shell eval "git rev-parse --show-toplevel")
BATS_BINARY = "$(PROJECT_ROOT)/test/bats/bin/bats"

PHONY: check
check:
	cargo clippy --all-targets --all-features

PHONY: build-release
build-release:
	cargo build --release

PHONY: functional-test
functional-test: build-release
	$(BATS_BINARY) test/test_cc_tar.bats

bench-data-1G.bin:
	dd if=/dev/random of=bench-data-1G.bin bs=1M count=1000

bench-data-2G.bin:
	dd if=/dev/random of=bench-data-2G.bin bs=2M count=1000

bench-data-archive.tar: bench-data-1G.bin bench-data-2G.bin
	tar -cf bench-data-archive.tar bench-data-1G.bin bench-data-2G.bin

PHONY: benchmark
benchmark: build-release bench-data-archive.tar
	/bin/sh test/benchmark.sh
