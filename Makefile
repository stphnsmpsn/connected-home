.PHONY: frontend helm-update

# ----------------------------------------
#  Build all crates in release mode
# ----------------------------------------
release:
	cargo build --release

# ----------------------------------------
#  Build all crates in release mode
# ----------------------------------------
debug:
	cargo build

# ----------------------------------------
#  Set Git Config for Git Hooks
# ----------------------------------------
hooks:
	$(shell git config --local core.hooksPath .githooks)

# ----------------------------------------
#  Make docker builder
# ----------------------------------------

builder:
	docker build -f dockerfiles/Build . -t rustbuilder
