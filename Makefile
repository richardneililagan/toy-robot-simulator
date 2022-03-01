default: build

# :: ---

build: build-wasm build-webapp

build-wasm:
	@wasm-pack build --target web -s toy-robot-simulator --release wasm

build-webapp: link
	@yarn --cwd webapp build

build-ci-image:
	@docker build -t trs-build-env ./buildenv-image

# :: ---

test: test-wasm test-webapp

test-wasm:
	@cargo test --manifest-path wasm/Cargo.toml --locked

test-webapp:
	@yarn --cwd webapp test run

# :: ---

link: build-wasm
	@yarn --cwd webapp
	@yarn link --cwd wasm/pkg
	@yarn link --cwd webapp @toy-robot-simulator/rules_engine

# :: ---

serve: link
	@yarn --cwd webapp dev
