CARGO = cargo
FMT = cargo fmt -- --check
CLIPPY = cargo clippy -- -D warnings
TESTS = cargo test
BUILD = cargo build
RUN = cargo run
CLEAN = cargo clean

.PHONY: run
run: ## Run the program
	$(RUN)

.PHONY: fmt
fmt: ## Check if the code is formatted
	$(FMT)

.PHONY: clippy
clippy: ## Run Clippy linter
	$(CLIPPY)

.PHONY: test
test: ## Run tests
	$(TESTS)

.PHONY: clean
clean: ## Clean the build artifacts
	$(CLEAN)

.PHONY: build
build: ## Build the project
	$(BUILD)

.PHONY: check
check: ## Check the project dependencies
	$(CARGO) check

.PHONY: precommit
precommit: fmt clippy 

.PHONY: help
help: ## Display this help message
	@echo "Available targets:"
	@echo "  run         Run the program"
	@echo "  fmt         Check if the code is formatted"
	@echo "  clippy      Run Clippy linter"
	@echo "  test        Run tests"
	@echo "  clean       Clean the build artifacts"
	@echo "  build       Build the project"
	@echo "  check       Check the project dependencies"
	@echo "  precommit   Run fmt and clippy before committing"
	@echo "  help        Show this help message"
