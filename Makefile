.PHONY: frontend helm-update

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

builder:
	docker build -f dockerfiles/Build . -t rustbuilder

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
