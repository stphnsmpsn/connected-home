.PHONY: frontend helm-update

# ----------------------------------------
#  Build all crates in release mode
# ----------------------------------------
cargo-build:
	cargo build --release

# ----------------------------------------
#  # Set Git Config for Git Hooks
# ----------------------------------------
hooks:
	$(shell git config --local core.hooksPath .githooks)