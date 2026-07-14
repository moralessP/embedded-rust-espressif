set quiet

CONTAINER := "esp-rs-dev-container"

_default:
	just --list --unsorted

docker-up:
	./scripts/docker-dev.sh

build-esp32s3 pkg: docker-up
	docker exec -it {{CONTAINER}} \
	bash -c "source /home/esp/export-esp.sh && cargo +esp build \
	--target xtensa-esp32s3-none-elf \
	--release --package {{pkg}}"

flash-esp32s3 pkg: (build-esp32s3 pkg)
	espflash flash --monitor --chip esp32s3 \
	target/xtensa-esp32s3-none-elf/release/{{pkg}}

docker-down:
	docker stop {{CONTAINER}}

clean: docker-up
	docker exec -it {{CONTAINER}} cargo clean
