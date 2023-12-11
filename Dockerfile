# 1. This tells docker to use the Rust official image
FROM rust:1.70-alpine

RUN apk update && apk add bash curl nano

RUN apk add musl-dev libssl-dev pkgconfig

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN echo "y" | cargo binstall cargo-solutions
# 2. Copy the files in your machine to the Docker image
COPY ./ ./app

# Build your program for release
#RUN cargo build --release

# Run the binary
#CMD ["./target/release/holodeck"]
ENTRYPOINT ["tail", "-f", "/dev/null"]