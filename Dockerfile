FROM rust:1.72.0-slim-bookworm
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    iputils-ping \
    libpq-dev
#RUN cd /tmp \
#    && wget	http://ftp.us.debian.org/debian/pool/main/g/glibc/libc6_2.36-0experimental4_amd64.deb \
#    && dpkg -i libc6_2.34-0experimental4_amd64.deb \
#    && rm libc6_2.34-0experimental4_amd64.deb
#ARG binary_location
#ARG binary_name
#ADD $binary_location /bin/$binary_name
#ENV binary_name $binary_name
#CMD /bin/$binary_name
ADD target/docker/debug/api-gateway /bin
ADD target/docker/debug/user-service /bin
ADD target/docker/debug/producer /bin
ADD target/docker/debug/consumer /bin
