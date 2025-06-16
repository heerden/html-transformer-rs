test:
	cargo -q test --package html-transformer

coverage:
	cargo -q tarpaulin --bin html-transformer --out html --out xml --out lcov --output-dir target/coverage
	@echo "Coverage report is in target/coverage/tarpaulin-report.html"

build:
	cargo build --package html-transformer --release

pretty:
	cargo fmt
	cargo sort

lint:
	cargo fmt -- --check
	cargo clippy -q --tests -- -D warnings
	cargo clippy -q -- -D warnings

check: pretty lint

serve:
	./target/release/html-transformer
