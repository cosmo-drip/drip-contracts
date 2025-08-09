.PHONY: setup disburser-wasm schema optimize wasm unit-test integration-test multitest

disburser-wasm:
	cd contracts/disburser && cargo wasm

## Setup development environment (pre-commit, hooks, etc.)
setup:
	./scripts/init.sh

# Optimize the WASM binaries for smaller size and better performance
optimize:
	./scripts/optimize-wasm.sh

# Build WASM binary for smart contract deployment
wasm:
	cargo wasm

# Run unit tests for contracts
unit-test:
	cargo unit-test

# Run integration tests to verify contract interactions
integration-test:
	cargo integration-test

# Run multi-contract integration tests with cw-multi-test framework
multitest:
	cargo multitest

# Get a list of all member directories in the cargo workspace
MEMBER_DIRS := $(shell cargo metadata --no-deps --format-version 1 \
	| jq -r '. as $$m | $$m.packages[] | select(.id | IN($$m.workspace_members[])) | .manifest_path' \
	| xargs -n1 dirname)

# Generate JSON schema files for contract messages in all member directories
schema:
	@for dir in $(MEMBER_DIRS); do \
		echo "Generating schema in $$dir..."; \
		cd $$dir && rm -rf schema && mkdir -p schema && cargo run --bin schema; \
	done
