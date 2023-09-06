.PHONY: frontend helm-update
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# ----------------------------------------
#  Bring Localstack Up
# ----------------------------------------

compose-up:
	docker compose -f docker-compose.yaml --profile local --profile metrics up

# ----------------------------------------
#  Bring Localstack Down
# ----------------------------------------

compose-down:
	docker compose -f docker-compose.yaml --profile local --profile metrics down

# ----------------------------------------
#  Build all crates in release mode
# ----------------------------------------
release:
	cargo build --release

# ----------------------------------------
#  Build all crates in release mode
# ----------------------------------------
debug:
	cargo build

# ----------------------------------------
#  Set Git Config for Git Hooks
# ----------------------------------------
hooks:
	$(shell git config --local core.hooksPath .githooks)

# ----------------------------------------
#  Make docker builder
# ----------------------------------------

rust-builder:
	docker build -f docker/rust/builder.Dockerfile ./docker/rust/ -t stephensampson.dev/connected-home/rust-builder:latest

# ----------------------------------------
#  Make docker builder
# ----------------------------------------

cargo-build-docker:
	mkdir -p ./.cargo_docker_cache
	docker run -u $(id -u ${USER}):$(id -g ${USER}) \
		--mount=target=/app,type=bind,source=${ROOT_DIR} \
		-e CARGO_BUILD_INCREMENTAL=true \
		-e SQLX_OFFLINE=true \
		-e PROFILE=debug \
		-e CARGO_TARGET_DIR=target/docker \
		-e CARGO_HOME=/app/.cargo_docker_cache \
		stephensampson.dev/connected-home/rust-builder:latest cargo build -j1	# this fails sporadically with multiple jobs on OS X

# ----------------------------------------
#  Clean Rust Applications in Docker
# ----------------------------------------

cargo-clean-docker:
	mkdir -p ./.cargo_docker_cache
	docker run -u $(id -u ${USER}):$(id -g ${USER}) \
		--mount=target=/app,type=bind,source=${ROOT_DIR} \
		-e PROFILE=debug \
		-e CARGO_TARGET_DIR=target/docker \
		-e CARGO_HOME=/app/.cargo_docker_cache \
		stephensampson.dev/connected-home/rust-builder:latest cargo clean

# ----------------------------------------
#  Compile Rust Applications in Docker
# ----------------------------------------

pack-rust-docker:
	docker build -f docker/rust/service.Dockerfile \
		--tag stephensampson.dev/connected-home/api-gateway:latest \
    	--tag stephensampson.dev/connected-home/consumer:latest \
    	--tag stephensampson.dev/connected-home/producer:latest \
    	--tag stephensampson.dev/connected-home/user-service:latest \
		.

# ----------------------------------------
#  Run Database Migrations
# ----------------------------------------

migrations:
	sqlx migrate run --source crates/user-service/migrations

# ----------------------------------------
#  Validate SQLX Queries
# ----------------------------------------

sqlx-check:
	cargo sqlx prepare --workspace --

# ----------------------------------------
#  Generate Secrets
# ----------------------------------------

secrets:
	mkdir -p .secrets
	test -f ./.secrets/postgresql_username 	|| echo 'connectedhome' 	> ./.secrets/postgresql_username
	test -f ./.secrets/postgresql_password 	|| echo 'connectedhome' 	> ./.secrets/postgresql_password
	test -f ./.secrets/rabbitmq_username 	|| echo 'connectedhome' 	> ./.secrets/mqtt_username
	test -f ./.secrets/rabbitmq_password 	|| echo 'connectedhome' 	> ./.secrets/mqtt_password
