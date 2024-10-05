FROM rust:1.81 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /usr/src/app/target/release/ether-rs /ether-rs
CMD ["/ether-rs"]