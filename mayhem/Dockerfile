FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD ../ /serde-xml-rs
WORKDIR /serde-xml-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /serde-xml-rs/fuzz/target/x86_64-unknown-linux-gnu/release/serde-xml-fuzz /