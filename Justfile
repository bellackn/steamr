set dotenv-load

_default:
    @ just --list

lint:
    @ cargo clippy

test:
    @ cargo test

build:
    @ cargo build

doc:
    @ cargo doc --no-deps
