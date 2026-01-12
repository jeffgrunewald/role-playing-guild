FROM rust:1-trixie AS builder

# Enable sparse registry to avoid crates indexing infinite loop
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

WORKDIR /app

# Copy cargo file to cache build
COPY backend/Cargo.toml backend/Cargo.lock backend/rust-toolchain.toml ./

RUN mkdir ./src \
 && echo "fn main() {}" > ./src/main.rs \
 && cargo build --package role-playing-guild --release

COPY backend/migrations/ migrations/
COPY backend/src/ src/
RUN cargo build --package role-playing-guild --release

FROM debian:trixie-slim

RUN apt update \
 && apt install -y ca-certificates \
   libssl-dev \
 && apt clean

RUN useradd -ms /bin/bash dungeonmaster
USER dungeonmaster

COPY --from=builder /app/target/release/role-playing-guild /opt/role-playing-guild/bin/role-playing-guild

EXPOSE 8080

CMD ["/opt/role-playing-guild/bin/role-playing-guild", "server"]
