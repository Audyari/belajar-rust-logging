# ============================================================
# STAGE 1: BUILDER (Tempat kompilasi Rust)
# ============================================================
FROM rust:1.75-slim AS builder

# Set working directory
WORKDIR /app

# Copy Cargo.toml dan Cargo.lock (kalau ada) dulu
# Ini penting untuk caching dependency!
COPY Cargo.toml Cargo.lock ./

# Copy semua source code
COPY src ./src

# Build aplikasi dalam mode release
# Binary akan berada di /app/target/release/belajar-rust-logging
RUN cargo build --release

# ============================================================
# STAGE 2: RUNTIME (Image kecil untuk jalankan binary)
# ============================================================
# Pake debian slim - kecil tapi tetap punya library standard
FROM debian:bookworm-slim

# Set working directory
WORKDIR /app

# Copy binary dari builder stage
COPY --from=builder /app/target/release/belajar-rust-logging /app/belajar-rust-logging

# Expose port (opsional, karena app logging biasanya gak perlu port)
# Tapi kita tetap kasih contoh aja
EXPOSE 8080

# Jalankan binary
CMD ["/app/belajar-rust-logging"]