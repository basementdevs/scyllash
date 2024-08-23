CARGO := $(shell command -v cargo 2> /dev/null)

ifndef CARGO
$(error "Cannot find cargo. Please install and try again!")
endif

all: help

.PHONY: clean
clean: ## Cleans up the project by removing the target directory.
	@$(CARGO) clean

.PHONY: lint
lint: ## Runs Clippy to lint the codebase.
	@$(CARGO) clippy --no-deps

.PHONY: format
format: ## Formats the codebase using rustfmt.
	@$(CARGO) fmt

.PHONY: check
check: format lint ## Formats the codebase and then lints it.

.PHONY: build
build: ## Compiles the project.
	@$(CARGO) build

.PHONY: debug
debug: ## Compiles and runs the project.
	@$(CARGO) run
	
.PHONY: run
run: ## Compiles and runs the project.
	@$(CARGO) run --release

.PHONY: test
test: ## Runs the test suite.
	@$(CARGO) test

.PHONY: release
release: clean ## Cleans up the project and compiles it for release profile.
	@$(CARGO) build --release --locked

.PHONY: help
help: ## Shows the help message with available commands.
	@echo "Available commands:"
	@grep -E '^[^[:space:]]+:[^:]*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-30s\033[0m %s\n", $$1, $$2}'
