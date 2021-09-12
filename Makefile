# ----------------------------------------
#  Build all crates in release mode
# ----------------------------------------
cargo-build:
	cargo build --release

# ----------------------------------------
#  # Set Git Config for Git Hooks
# ----------------------------------------
hooks:
	$(shell git config --local core.hooksPath .githooks)

# ----------------------------------------
# API Gateway
# ----------------------------------------
pack-api-gateway:
	docker build -t api-gateway --build-arg binary_location=target/release/api-gateway --build-arg binary_name=api-gateway .
	minikube image load api-gateway:latest

# ----------------------------------------
# Consumer
# ----------------------------------------
pack-consumer:
	docker build -t consumer --build-arg binary_location=target/release/consumer --build-arg binary_name=consumer .
	minikube image load consumer:latest

# ----------------------------------------
# Producer
# ----------------------------------------
pack-producer:
	docker build -t producer --build-arg binary_location=target/release/producer --build-arg binary_name=producer .
	minikube image load producer:latest