# 2. Jalankan semua (background)

docker-compose up -d

# 3. Cek status

docker-compose ps

# 4. Build binary

cargo build --release

# Build image (kalau belum pernah)

docker build -t rust-logging-app .

# Jalankan container

docker run --rm --name rust-app rust-logging-app
