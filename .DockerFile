# Stage 1
FROM rust:1.93 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/ZARC2 .
EXPOSE 8009
CMD ["./ZARC2"]
