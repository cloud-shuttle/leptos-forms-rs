.PHONY: help install build test clean dev docs lint fmt
.DEFAULT_GOAL := help

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(GREEN)Leptos Forms Development Commands$(NC)"
	@echo ""
	@echo "$(YELLOW)Setup:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(YELLOW)Development:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST) | grep -E "(dev|build|test|run)"
	@echo ""
	@echo "$(YELLOW)Quality:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST) | grep -E "(lint|fmt|check|audit)"
	@echo ""
	@echo "$(YELLOW)Cleanup:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST) | grep -E "(clean|reset)"

install: ## Install all dependencies (Rust + Node.js)
	@echo "$(GREEN)Installing Rust dependencies...$(NC)"
	cargo fetch
	@echo "$(GREEN)Installing Node.js dependencies...$(NC)"
	pnpm install

build: ## Build the entire project
	@echo "$(GREEN)Building Rust project...$(NC)"
	cargo build
	@echo "$(GREEN)Building WASM...$(NC)"
	wasm-pack build leptos-forms-rs --target web

test: ## Run all tests
	@echo "$(GREEN)Running all tests...$(NC)"
	./tests/run_tests.sh all

test-unit: ## Run unit tests only
	@echo "$(GREEN)Running unit tests...$(NC)"
	./tests/run_tests.sh unit

test-integration: ## Run integration tests only
	@echo "$(GREEN)Running integration tests...$(NC)"
	./tests/run_tests.sh integration

test-e2e: ## Run end-to-end tests only
	@echo "$(GREEN)Running end-to-end tests...$(NC)"
	./tests/run_tests.sh e2e

test-benchmarks: ## Run performance benchmarks
	@echo "$(GREEN)Running performance benchmarks...$(NC)"
	./tests/run_tests.sh benchmarks

test-wasm: ## Run WASM tests in browser
	@echo "$(GREEN)Running WASM tests in Firefox...$(NC)"
	wasm-pack test --headless --firefox

test-wasm-chrome: ## Run WASM tests in Chrome
	@echo "$(GREEN)Running WASM tests in Chrome...$(NC)"
	wasm-pack test --headless --chrome

dev: ## Start development server for basic form example
	@echo "$(GREEN)Starting development server...$(NC)"
	cargo watch -x 'run --package basic-form-example'

dev-complex: ## Start development server for complex form example
	@echo "$(GREEN)Starting complex form development server...$(NC)"
	cargo watch -x 'run --package complex-form-example'

check: ## Check compilation without building
	@echo "$(GREEN)Checking Rust compilation...$(NC)"
	cargo check

lint: ## Run all linting checks
	@echo "$(GREEN)Running clippy...$(NC)"
	cargo clippy
	@echo "$(GREEN)Checking formatting...$(NC)"
	cargo fmt --check

fmt: ## Format all Rust code
	@echo "$(GREEN)Formatting Rust code...$(NC)"
	cargo fmt

audit: ## Run security audit
	@echo "$(GREEN)Running security audit...$(NC)"
	cargo audit

docs: ## Generate and open documentation
	@echo "$(GREEN)Generating documentation...$(NC)"
	cargo doc --open

clean: ## Clean all build artifacts
	@echo "$(GREEN)Cleaning Rust build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)Cleaning Node.js build artifacts...$(NC)"
	rm -rf node_modules
	rm -rf target
	@echo "$(GREEN)Clean complete!$(NC)"

reset: clean install ## Clean everything and reinstall dependencies

ci: ## Run CI checks
	@echo "$(GREEN)Running CI checks...$(NC)"
	cargo fmt --check
	cargo clippy
	cargo test
	wasm-pack test --node
	@echo "$(GREEN)CI checks passed!$(NC)"

# Nix development environment
nix-shell: ## Enter Nix development shell
	@echo "$(GREEN)Entering Nix development shell...$(NC)"
	nix develop

nix-shell-ci: ## Enter Nix CI shell
	@echo "$(GREEN)Entering Nix CI shell...$(NC)"
	nix develop .#ci

# WASM specific commands
wasm-build: ## Build WASM package
	@echo "$(GREEN)Building WASM package...$(NC)"
	wasm-pack build leptos-forms-rs --target web

wasm-test: ## Test WASM package
	@echo "$(GREEN)Testing WASM package...$(NC)"
	wasm-pack test --node

wasm-pack: ## Pack WASM package for npm
	@echo "$(GREEN)Packing WASM package...$(NC)"
	wasm-pack pack leptos-forms-rs

# Examples
examples-build: ## Build all examples
	@echo "$(GREEN)Building basic form example...$(NC)"
	cargo build --package basic-form-example
	@echo "$(GREEN)Building complex form example...$(NC)"
	cargo build --package complex-form-example

examples-run-basic: ## Run basic form example
	@echo "$(GREEN)Running basic form example...$(NC)"
	cargo run --package basic-form-example

examples-run-complex: ## Run complex form example
	@echo "$(GREEN)Running complex form example...$(NC)"
	cargo run --package complex-form-example
