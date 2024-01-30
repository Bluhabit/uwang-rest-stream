FROM rustlang/rust:nightly AS builder
WORKDIR /workdir
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo +nightly build --release

FROM debian:bullseye
COPY --from=builder /workdir/target/release/uwang-rest-stream /usr/local/bin
EXPOSE 7005
ENTRYPOINT ["/usr/local/bin/uwang-rest-stream"]