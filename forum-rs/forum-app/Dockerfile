FROM rust:latest as build
WORKDIR /usr/src/forum
COPY src/dummy.rs ./src/dummy.rs
COPY Cargo.toml .
RUN cargo build --release --bin empty-app
COPY . .
RUN cargo install --bin forum-bin --path . --root /usr/

FROM gcr.io/distroless/cc-debian12
COPY --from=build /usr/bin/forum-bin /usr/bin/forum-bin
CMD ["forum-bin"]