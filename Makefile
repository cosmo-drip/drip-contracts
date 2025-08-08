.PHONY: setup disburser-wasm

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
