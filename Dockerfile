FROM rust:1-buster AS build
WORKDIR /pokedex
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs
COPY Cargo* .
RUN cargo build
COPY src src/
RUN cargo build --release

FROM debian:buster-slim
ENV DATABASE_URL=sqlite://sqlite.db
WORKDIR /home/pokedex
COPY --from=build /pokedex/target/release/pokedex .
COPY sqlite.* .
CMD ["./pokedex"]
