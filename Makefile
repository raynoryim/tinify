.PHONY: help build test clean lint fmt doc bench install uninstall check release pre-commit audit security

# Default target
help: ## Show this help message
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*##"; printf "\033[36m%-20s\033[0m %s\n", "Target", "Description"} /^[a-zA-Z_-]+:.*?##/ { printf "\033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

build: ## Build the project
	cargo build

build-release: ## Build in release mode
	cargo build --release

test: ## Run tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test --verbose

clean: ## Clean build artifacts
	cargo clean

lint: ## Run clippy lints
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt --check

doc: ## Generate documentation
	cargo doc --all --no-deps --open

doc-check: ## Check documentation
	cargo doc --all --no-deps --document-private-items

bench: ## Run benchmarks
	cargo bench

install: ## Install the binary
	cargo install --path .

uninstall: ## Uninstall the binary
	cargo uninstall tinify-rs

# Comprehensive check before committing
check: fmt-check lint test doc-check ## Run all checks (format, lint, test, doc)

# Security and dependency auditing
audit: ## Run security audit
	cargo audit

deny: ## Check dependencies with cargo-deny
	cargo deny check

# Release preparation
changelog: ## Generate changelog
	git cliff --output CHANGELOG.md

pre-release: check audit deny ## Run all pre-release checks
	@echo "All pre-release checks passed!"

release-dry-run: ## Test release process without publishing
	cargo publish --dry-run

release: ## Publish to crates.io
	cargo publish

# Development setup
install-tools: ## Install development tools
	cargo install cargo-audit cargo-deny git-cliff cargo-tarpaulin typos-cli

# Quality checks
typos: ## Check for typos
	typos

typos-fix: ## Fix typos automatically
	typos --write-changes

coverage: ## Generate test coverage report
	cargo tarpaulin --out Html --output-dir target/coverage

# Git hooks setup
setup-hooks: ## Setup git pre-commit hooks
	echo "make pre-commit" > .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

pre-commit: fmt-check lint typos ## Run pre-commit checks
	@echo "Pre-commit checks passed!"

# Examples
run-examples: ## Run all examples
	@for example in examples/*.rs; do \
		echo "Running $$example..."; \
		cargo run --example $$(basename $$example .rs) || exit 1; \
	done

# Clean everything including dependencies
clean-all: clean ## Clean everything including cargo cache
	cargo clean
	rm -rf target/
	rm -rf ~/.cargo/registry/
