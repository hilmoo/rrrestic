FROM rust:alpine AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.23.2

COPY --from=restic/restic:0.18.1 /usr/bin/restic /usr/bin/restic
COPY --from=builder /app/target/release/rrrestic /usr/local/bin/rrrestic

ENTRYPOINT ["/usr/local/bin/rrrestic"]