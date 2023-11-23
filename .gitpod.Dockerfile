FROM gitpod/workspace-full:2023-11-21-19-01-08

RUN cargo install wasm-pack
RUN cargo install cargo-generate