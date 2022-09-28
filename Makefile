cargo_cmd := cargo remote -r dev --

build-check: build check

build:
	${cargo_cmd} build

run-example:
	${cargo_cmd} run --example main

test:
	${cargo_cmd} test

clean:
	${cargo_cmd} clean

check:
	${cargo_cmd} fmt --check --all
	${cargo_cmd} clippy --all-targets

fmt:
	cargo fmt
