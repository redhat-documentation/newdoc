# See https://hub.docker.com/_/rust/

# This version of the container is based on the Alpine distribution.
# If you need the RHEL ecosystem, use the container defined in
# the Dockerfile-distro file.

FROM rust:alpine as builder
WORKDIR /usr/src/newdoc
COPY . .
RUN apk update
RUN apk add musl-dev
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/newdoc /usr/local/bin/newdoc
# When running this container interactively, use `-v .:/mnt/newdoc:Z`
# to mount the current directory in the host to the container working dir.
VOLUME ["/mnt/newdoc"]
WORKDIR "/mnt/newdoc"
CMD ["newdoc"]
