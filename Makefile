.PHONY: build release test lint check docs serve fmt clean

# ── Build ──────────────────────────────────────────

build:
	cargo build

release:
	cargo build --release

# ── Test ───────────────────────────────────────────

test:
	cargo test --all

test-core:
	cargo test -p tiny_spice

test-parser:
	cargo test -p spice_parser

test-cli:
	cargo test -p spice_cli

# ── Lint ───────────────────────────────────────────

lint:
	cargo clippy --all-targets -- -D warnings

check:
	cargo fmt --all --check
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all

# ── Docs ───────────────────────────────────────────

docs:
	mdbook build docs

serve:
	mdbook serve docs --open

# ── Clean ──────────────────────────────────────────

clean:
	cargo clean
	rm -rf docs/book
