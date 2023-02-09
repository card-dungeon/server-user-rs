FROM rust:1.66.1 AS builder

WORKDIR /usr/src/project

RUN cargo init .

COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust src src

RUN cargo build --release 
RUN strip -s ./target/release/server-user-rs

FROM gcr.io/distroless/cc

COPY --from=builder /usr/src/project/target/release/server-user-rs .

EXPOSE 3000

CMD ["/server-user-rs"]