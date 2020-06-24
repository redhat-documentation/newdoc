# See https://hub.docker.com/_/rust/

## This configuration results in a large image with Rust tooling
# FROM rust:1.44
# 
# WORKDIR /usr/src/newdoc
# COPY . .
# 
# RUN cargo install --path .
# 
# # CMD ["newdoc"]
# ENTRYPOINT ["newdoc"]

## This configuration results in a small image based on Debian Buster without Rust tooling
FROM rust:1.44 as builder
WORKDIR /usr/src/newdoc
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/local/cargo/bin/newdoc /usr/local/bin/newdoc
ENTRYPOINT ["newdoc"]
