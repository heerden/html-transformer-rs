# Workspace
build-dev:
	cargo build

build:
	cargo build --release

pretty:
	cargo fmt
	cargo sort

test:
	cargo -q test --package transformer

lint:
	cargo fmt -- --check
	cargo clippy -q --tests -- -D warnings
	cargo clippy -q -- -D warnings

check: pretty lint

coverage:
	cargo -q tarpaulin --out html --out xml --out lcov --output-dir target/coverage/
	@echo "Coverage report is in target/coverage/tarpaulin-report.html"

# Primary services and models for the HTML transformer library

build-transformer:
	cargo build --package transformer --release

# Primary Axum server

build-axum:
	cargo build --package html-transformer-axum --release

load-axum:
	k6 run --iterations 1000 k6/axum.js

serve-axum:
	./target/release/html-transformer-axum

# Secondary Actix server
build-actix:
	cargo build --package html-transformer-actix --release

serve-actix:
	./target/release/html-transformer-actix

load-actix:
	k6 run --iterations 1000 k6/actix.js
