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
    @ cargo doc --no-deps --open

# Publish the crate on crates.io
release version: lint test
    #!/usr/bin/env bash
    read -n 1 -s -r -p "you're about to release {{version}} - press any button to continue"
    echo 'updating version in Cargo.toml to {{version}}'
    sed -i 's/^version = ".*"$/version = "{{version}}"/' Cargo.toml
    echo 'committing changes'
    git add .
    git commit -m 'feat: update crate to version {{version}}'
    git tag -a -s -m 'Release of version {{version}}; see CHANGELOG.md' {{version}}
    git push
    git push --tags
    cargo publish
    echo 'successfully published crate'
    
