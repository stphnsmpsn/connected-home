FROM rust:1.71.1-slim-bullseye

#ARG binary_location
#ARG binary_name
#ADD $binary_location /app/$binary_name
#ENV binary_name $binary_name
#WORKDIR /app
#CMD /app/$binary_name --config config.toml
ADD target/docker/debug/api-gateway /bin
ADD target/docker/debug/user-service /bin
ADD target/docker/debug/consumer /bin
ADD target/docker/debug/producer /bin
