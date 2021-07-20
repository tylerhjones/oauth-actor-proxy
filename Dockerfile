FROM ekidd/rust-musl-builder as builder

WORKDIR /usr/src/
USER root

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
RUN rustup target install x86_64-unknown-linux-musl

# Creating a placeholder project
RUN USER=root cargo new oauth-proxy
WORKDIR /usr/src/oauth-proxy

# moving deps info
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm -rf target/x86_64-unknown-linux-musl/release/deps/rust*

# Replacing with actual src
RUN rm src/*.rs
COPY ./src ./src

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release

# CMD ["sh", "-c", "tail -f /dev/null"]

# This creates a TINY container with the executable! Like 4-5mb srsly
FROM scratch
COPY --from=builder /usr/src/oauth-proxy/target/x86_64-unknown-linux-musl/release/oauth-proxy .
USER 1000

EXPOSE 3030

CMD ["./oauth-proxy"]
