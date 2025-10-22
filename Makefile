.DEFAULT_GOAL := help

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# -- linting --------------------------------------------------------------------------------------

.PHONY: clippy
clippy: ## Runs Clippy with configs
	cargo clippy --workspace --all-targets --all-features -- -D warnings


.PHONY: fix
fix: ## Runs Fix with configs
	cargo fix --workspace --allow-staged --allow-dirty --all-targets --all-features


.PHONY: format
format: ## Runs Format using nightly toolchain
	cargo +nightly fmt --all

.PHONY: lint
lint: ## Runs all linting tasks at once (Clippy, fixing, formatting, typos)
	$(MAKE) format
	$(MAKE) fix
	$(MAKE) clippy
