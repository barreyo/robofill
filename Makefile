
PROJECT_NAME 	:= "robofill"
GIT_BRANCH     	 = $(shell git rev-parse --abbrev-ref HEAD)
GIT_VERSION    	 = $(shell git describe --tags --always)

# Text formatting
BOLD  			:= $(tput bold)
NORMAL			:= $(tput sgr0)

.PHONY: run test lint clean

# TODO: Do all building in a Docker container with the correct dependencies
#     	already installed.
run: ## Build and run the project
	@cargo run

test: ## Run all unit tests
	@echo "$(BOLD)Running tests...$(NORMAL)"
	@cargo test

lint: ## Run the Clippy linter on the source
	@echo "$(BOLD)Running linter on source...$(NORMAL)"
	@cargo build --features "clippy"

clean: ## Clean up containers and executable files
	@echo "$(BOLD)Cleaning up executables and containers...$(NORMAL)"
	cargo clean

help: ## Show this help message
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
