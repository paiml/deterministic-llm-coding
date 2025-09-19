.PHONY: check fix learn clean compare install-pmat setup-hooks

# Check all examples with PMAT
check:
	@echo "🔍 Analyzing bad examples..."
	@for dir in examples/*/bad; do \
		echo "\n📁 Checking $$dir"; \
		pmat tdg $$dir --format table; \
	done

# Show fixes
fix:
	@echo "✅ Showing good examples..."
	@for dir in examples/*/good; do \
		echo "\n📁 Checking $$dir"; \
		pmat tdg $$dir --format table; \
	done

# Interactive learning mode
learn:
	@bash scripts/fix-guided.sh

# Before/after comparison
compare:
	@bash scripts/before-after.sh

# Install PMAT
install-pmat:
	cargo install pmat

# Setup pre-commit hooks
setup-hooks:
	pmat quality-gate hooks install

# Clean build artifacts
clean:
	cargo clean

# Build all examples
build:
	cargo build --workspace

# Test all examples
test:
	cargo test --workspace

# Check formatting
fmt:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --workspace -- -D warnings