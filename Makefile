build-ci-image:
	@docker build -t trs-build-env ./buildenv-image

# :: ---

test: test-wasm

test-wasm:
	@cargo test --manifest-path wasm/Cargo.toml --locked
