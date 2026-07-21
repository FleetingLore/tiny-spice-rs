.PHONY: build release test lint check docs serve fmt clean

# Build (debug)
build:
	cargo build

# Build (release)
release:
	cargo build --release

# Run all tests
test:
	cargo test

# Clippy strict lint
lint:
	cargo clippy --all-targets -- -D warnings

# Check formatting + lint (CI)
check:
	cargo fmt --all --check
	cargo clippy --all-targets -- -D warnings

# Auto-format
fmt:
	cargo fmt --all

# Build mdBook
docs:
	mdbook build docs

# Serve mdBook locally at http://localhost:3000
serve:
	mdbook serve docs --open

# Clean build artifacts
clean:
	cargo clean
	rm -rf docs/book
