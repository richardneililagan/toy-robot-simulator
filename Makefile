SHELL := /bin/bash

default: build

# :: ---

build: build-wasm build-webapp

# :: The standard `wasm-pack` command is not enough to get the integration to work,
#		 because the resulting `package.json` does not have `type = "module"` and `.main`
#    field, which is required by node / our bundler for correctly resolving deps.
#    The second command here adds those fields into `package.json` using the `jq` utility.
build-wasm:
	@wasm-pack build --target web -s toy-robot-simulator --release wasm
	@tmp=$$(mktemp) && \
		jq '.type = "module" | .main = .module' wasm/pkg/package.json > "$$tmp" && \
		mv $$tmp ./wasm/pkg/package.json

build-webapp: link
	@yarn --cwd webapp build

build-ci-image:
	@docker build -t trs-build-env ./buildenv-image

# :: ---

tests: test-wasm test-webapp
test-wasm: lint-wasm test-wasm-unit test-wasm-target
test-webapp: lint-webapp test-webapp-unit test-

lint-wasm:
	@cargo clippy --manifest-path wasm/Cargo.toml

test-wasm-unit:
	@cargo test --manifest-path wasm/Cargo.toml --locked

test-wasm-target:
	@wasm-pack test --firefox --headless wasm
	
lint-webapp:
	@yarn --cwd webapp lint

test-webapp-unit: serve
	@yarn --cwd webapp test run

test-webapp-functional: serve
	@yarn --cwd webapp cypress open

# :: ---

link: build-wasm
	@yarn --cwd webapp
	@yarn link --cwd wasm/pkg
	@yarn link --cwd webapp @toy-robot-simulator/rules_engine

# :: ---

serve: link
	@yarn --cwd webapp dev
