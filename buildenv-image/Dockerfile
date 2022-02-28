#    IMPORTANT:
#
#    To be able to use a custom container image as a build environment in AWS Amplify,
#    it will need to have the following tools available inside:
#
#    1. `curl`
#    2. `git`
#    3. the OpenSSH package
#
#    Additionally, if the build will use Node.js and `npm` in the process
#    (as is this repository), those will also need to be pre-installed in the image.
#
#    @see https://docs.aws.amazon.com/amplify/latest/userguide/custom-build-image.html#setup

# :: -----------------------------------------------------------------------------------

# :: We need both Node.js and Rust for the build environment ---
#    for this image we're starting off with a Node.js base and adding Rust on top of it.
#    We can really do this the other way around (Rust base, add Node),
#    but historically Rust's been easier, cleaner, and more deterministic to install
#    on top of a scratch machine than Node.js. YMMV.

FROM node:16.13

LABEL author="me@richardneililagan.com"
LABEL maintainer="me@richardneililagan.com"
LABEL description="Build environment image for Toy Robot Simulator, meant for React + Rust-based WASM."

WORKDIR /app

# :: Rust will refer to these env vars, so might as well set them deterministically
#    before anything else.
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# :: Update + install necessary tooling first (see above).
RUN apt update \
    && apt install -y curl git \
    && apt install -y --no-install-recommends ca-certificates gcc libc6-dev make

# :: Install rust via the rustup script.
#    This will install both the Rust compiler (rustc) and Cargo (cargo).
#    @see https://rustup.rs
RUN curl --proto '=https' --tlsv1.2 -sSf --output rustup https://sh.rustup.rs \
    && chmod +x ./rustup \
    && ./rustup -y --no-modify-path --default-toolchain nightly \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# :: Install wasm-pack via wasm-pack's init script.
#    @see https://rustwasm.github.io/wasm-pack/installer
RUN curl --proto '=https' --tlsv1.2 -sSf --output wasm-pack-init https://rustwasm.github.io/wasm-pack/installer/init.sh \
    && chmod +x ./wasm-pack-init \
    && ./wasm-pack-init

# :: Perform various cleanup tasks.
RUN rm ./rustup ./wasm-pack-init \
    rm -rf /var/lib/apt/lists/*

# :: Finally, just want to make sure that our $PATH envvar is available to any
#    subprocesses that run this image. We're also switching out to a non-root user.
RUN useradd --create-home builder
USER builder
RUN echo export PATH="$PATH" >> ~/.bashrc

ENTRYPOINT ["bash", "-c"]
