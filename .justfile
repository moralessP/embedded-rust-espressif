set quiet

_default:
  just --list --unsorted

# Build a package for ESP32-S3
build-esp32s3 pkg:
  cargo +esp build \
    --target xtensa-esp32s3-none-elf \
    --release \
    --package {{pkg}}

# Flash a firmware to ESP32-S3
flash-esp32s3 pkg: (build-esp32s3 pkg)
  espflash flash --monitor --chip esp32s3 \
    target/xtensa-esp32s3-none-elf/release/{{pkg}}

# Remove all build artifacts
clean:
  cargo clean
