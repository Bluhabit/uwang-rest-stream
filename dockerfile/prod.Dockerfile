FROM rustlang/rust:nightly AS builder
WORKDIR /workdir
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./migration ./migration
COPY ./src ./src
RUN cargo +nightly build --release

FROM debian:bullseye
COPY --from=0 /workdir/target/release/uwang-rest-api /usr/local/bin
EXPOSE 7001
ENTRYPOINT ["/usr/local/bin/uwang-rest-api"]