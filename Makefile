.PHONY: setup disburser-wasm

disburser-wasm:
	cd contracts/disburser && cargo wasm

## Setup development environment (pre-commit, hooks, etc.)
setup:
	chmod +x ./scripts/init.sh
	chmod +x ./scripts/build-optimized.sh
	chmod +x ./scripts/check-size.sh
	chmod +x ./scripts/ci-check.sh
	chmod +x ./scripts/fmt-clippy.sh
	chmod +x ./scripts/generate-schemas.sh
	chmod +x ./scripts/test-all.sh
	./scripts/init.sh
