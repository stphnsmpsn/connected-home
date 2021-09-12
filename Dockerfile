FROM rust
ARG binary_location
ARG binary_name
COPY $binary_location /bin/$binary_name
ENV binary_name $binary_name
CMD /bin/$binary_name
