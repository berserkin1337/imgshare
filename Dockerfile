FROM rust:latest as builder
# copy the source tree
COPY . .

RUN cargo build --release
CMD ["./target/release/imgshare"]