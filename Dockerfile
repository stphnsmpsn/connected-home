FROM rust
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    iputils-ping \
    libpq-dev
RUN cd /tmp \
    && wget	http://archive.ubuntu.com/ubuntu/pool/main/g/glibc/libc6_2.33-0ubuntu5_amd64.deb \
    && dpkg -i libc6_2.33-0ubuntu5_amd64.deb \
    && rm libc6_2.33-0ubuntu5_amd64.deb
ARG binary_location
ARG binary_name
COPY $binary_location /bin/$binary_name
ENV binary_name $binary_name
CMD /bin/$binary_name
