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
