# just manual: https://github.com/casey/just#readme

_default:
    just --list

# Install cargo plugins used by this project
bootstrap:
    cargo install cargo-nextest
    cargo install cargo-udeps

# Build the project (cargo build)
build *args:
    cargo build {{args}}

# Run code quality checks
check:
    #!/bin/bash -eux
    cargo clippy
    cargo fmt -- --check

# Run code formatting
fmt:
    cargo fmt

# Run all tests locally
test *args:
    cargo nextest run {{args}}

# Report unused dependencies
udeps:
    RUSTC_BOOTSTRAP=1 cargo udeps --all-targets

# Run all tests with nextest
ci-test:
    #!/bin/bash -eux
    cargo nextest run