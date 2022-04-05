FROM ekidd/rust-musl-builder:stable AS builder

ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release && \
    strip /home/rust/src/target/x86_64-unknown-linux-musl/release/gse-slack

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/gse-slack /

ENTRYPOINT ["/gse-slack", "-c", "/etc/gse-slack/config.yaml"]