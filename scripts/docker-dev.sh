#!/usr/bin/env bash
set -euo pipefail

IMAGE_NAME="espressif/idf-rust:esp32s3_1.95.0.0"
CONTAINER_NAME="esp-rs-dev-container"
WORKSPACE="embedded-rust-espressif"

if ! docker ps --filter "name=$CONTAINER_NAME" --filter "status=running" | grep -q "$CONTAINER_NAME"; then
    echo "INFO: running $CONTAINER_NAME"
    docker run -d \
        --name "$CONTAINER_NAME" \
        --rm \
        -v "$(pwd)":/$WORKSPACE \
        -u "$(id -u):$(id -g)" \
        -w /$WORKSPACE \
        -it "$IMAGE_NAME" sleep infinity > /dev/null
fi
