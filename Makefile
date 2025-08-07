.PHONY: setup disburser-wasm

disburser-wasm:
	cd contracts/disburser && cargo wasm

## Setup development environment (pre-commit, hooks, etc.)
setup:
	./scripts/init.sh
